// Generated macro for FileFormat (enum)
macro_rules! Depcrate_file_formatFileFormat {
() => {
// Module: crate::file::format
// Provides: {"FileFormat"}
// Dependencies: {}
# [doc = " File formats provided by the library."] # [doc = ""] # [doc = " Although it is possible to define custom formats using [`Format`] trait it is recommended to use `FileFormat` if possible."] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [non_exhaustive] pub enum FileFormat { # [doc = " TOML (parsed with toml)"] # [cfg (feature = "toml")] Toml , # [doc = " JSON (parsed with `serde_json`)"] # [cfg (feature = "json")] Json , # [doc = " YAML (parsed with `yaml_rust2`)"] # [cfg (feature = "yaml")] Yaml , # [doc = " INI (parsed with `rust_ini`)"] # [cfg (feature = "ini")] Ini , # [doc = " RON (parsed with ron)"] # [cfg (feature = "ron")] Ron , # [doc = " JSON5 (parsed with json5)"] # [cfg (feature = "json5")] Json5 , # [doc = " Corn (parsed with `libcorn`)"] # [cfg (feature = "corn")] Corn , }
};
}
