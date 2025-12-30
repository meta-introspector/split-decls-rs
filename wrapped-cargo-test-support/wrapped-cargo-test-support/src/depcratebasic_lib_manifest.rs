// Generated macro for basic_lib_manifest (function)
macro_rules! Depcratebasic_lib_manifest {
() => {
// Module: crate
// Provides: {"basic_lib_manifest"}
// Dependencies: {}
# [doc = " Generate a `Cargo.toml` with the specified `lib.name`"] pub fn basic_lib_manifest (name : & str) -> String { format ! (r#"
        [package]

        name = "{}"
        version = "0.5.0"
        authors = ["wycats@example.com"]
        edition = "2015"

        [lib]

        name = "{}"
    "# , name , name) }
};
}
