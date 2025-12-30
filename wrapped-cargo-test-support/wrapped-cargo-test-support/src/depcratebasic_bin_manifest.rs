// Generated macro for basic_bin_manifest (function)
macro_rules! Depcratebasic_bin_manifest {
() => {
// Module: crate
// Provides: {"basic_bin_manifest"}
// Dependencies: {}
# [doc = " Generate a `Cargo.toml` with the specified `bin.name`"] pub fn basic_bin_manifest (name : & str) -> String { format ! (r#"
        [package]

        name = "{}"
        version = "0.5.0"
        authors = ["wycats@example.com"]
        edition = "2015"

        [[bin]]

        name = "{}"
    "# , name , name) }
};
}
