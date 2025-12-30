// Generated macro for basic_manifest (function)
macro_rules! Depcratebasic_manifest {
() => {
// Module: crate
// Provides: {"basic_manifest"}
// Dependencies: {}
# [doc = " Generate a basic `Cargo.toml`"] pub fn basic_manifest (name : & str , version : & str) -> String { format ! (r#"
        [package]
        name = "{}"
        version = "{}"
        authors = []
        edition = "2015"
    "# , name , version) }
};
}
