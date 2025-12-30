// Generated macro for SourceList (struct)
macro_rules! Depcrate_inputSourceList {
() => {
// Module: crate::input
// Provides: {"SourceList"}
// Dependencies: {}
# [doc = " List of sources to check, loaded from a .toml file"] # [derive (Debug , Deserialize)] pub struct SourceList { crates : HashMap < String , TomlCrate > , # [serde (default)] recursive : RecursiveOptions , }
};
}
