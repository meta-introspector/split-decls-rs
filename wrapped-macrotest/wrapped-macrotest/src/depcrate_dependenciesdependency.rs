// Generated macro for Dependency (struct)
macro_rules! Depcrate_dependenciesDependency {
() => {
// Module: crate::dependencies
// Provides: {"Dependency"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Debug)] # [serde (remote = "Self")] pub struct Dependency { # [serde (skip_serializing_if = "Option::is_none")] pub version : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub path : Option < PathBuf > , # [serde (rename = "default-features" , default = "get_true" , skip_serializing_if = "is_true")] pub default_features : bool , # [serde (default , skip_serializing_if = "Vec::is_empty")] pub features : Vec < String > , # [serde (default , skip_serializing_if = "is_false")] pub workspace : bool , # [serde (flatten)] pub rest : Map < String , Value > , }
};
}
