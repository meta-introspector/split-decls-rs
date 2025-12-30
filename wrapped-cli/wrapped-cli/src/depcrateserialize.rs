// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
fn serialize (config : & Value , output_type : OutputType) -> Result < String , Error > { match output_type { OutputType :: Json => serde_json :: to_string_pretty (& config) . map_err (Error :: from) , OutputType :: Yaml => serde_norway :: to_string (& config) . map_err (Error :: from) , OutputType :: Toml => toml_edit :: ser :: to_string_pretty (& config) . map_err (Error :: from) , } }
};
}
