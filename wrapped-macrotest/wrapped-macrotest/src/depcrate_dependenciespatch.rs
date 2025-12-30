// Generated macro for Patch (struct)
macro_rules! Depcrate_dependenciesPatch {
() => {
// Module: crate::dependencies
// Provides: {"Patch"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Debug)] pub struct Patch { # [serde (skip_serializing_if = "Option::is_none")] pub path : Option < PathBuf > , # [serde (skip_serializing_if = "Option::is_none")] pub git : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub branch : Option < String > , }
};
}
