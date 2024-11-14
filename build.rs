use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/testit.c");
    println!("cargo:rerun-if-changed=src/testit.h");

    cc::Build::new().file("src/testit.c").compile("testit");

    let bindings = bindgen::Builder::default()
        .header("src/testit.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
