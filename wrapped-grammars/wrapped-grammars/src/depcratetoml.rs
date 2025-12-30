// Generated macro for toml (module)
macro_rules! Depcratetoml {
() => {
// Module: crate
// Provides: {"toml"}
// Dependencies: {}
# [doc = " Grammar rules of a sample TOML parser"] # [allow (missing_docs)] pub mod toml { # [doc = " TOML parser."] # [derive (Parser)] # [grammar = "grammars/toml.pest"] pub struct TomlParser ; }
};
}
