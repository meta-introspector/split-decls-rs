// Generated macro for Manifest (struct)
macro_rules! Depcrate_manifestManifest {
() => {
// Module: crate::manifest
// Provides: {"Manifest"}
// Dependencies: {}
# [derive (Serialize , Debug)] pub struct Manifest { # [serde (rename = "cargo-features")] # [serde (skip_serializing_if = "Vec::is_empty")] pub cargo_features : Vec < String > , pub package : Package , # [serde (skip_serializing_if = "Map::is_empty")] pub features : Map < String , Vec < String > > , pub dependencies : Map < String , Dependency > , # [serde (rename = "bin")] pub bins : Vec < Bin > , # [serde (skip_serializing_if = "Option::is_none")] pub workspace : Option < Workspace > , # [serde (skip_serializing_if = "Map::is_empty")] pub patch : Map < String , RegistryPatch > , # [serde (skip_serializing_if = "Map::is_empty")] pub replace : Map < String , Patch > , }
};
}
