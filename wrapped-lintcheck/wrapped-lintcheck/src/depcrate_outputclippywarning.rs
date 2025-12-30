// Generated macro for ClippyWarning (struct)
macro_rules! Depcrate_outputClippyWarning {
() => {
// Module: crate::output
// Provides: {"ClippyWarning"}
// Dependencies: {}
# [doc = " A single warning that clippy issued while checking a `Crate`"] # [derive (Debug , PartialEq , Eq , Hash , Serialize , Deserialize)] pub struct ClippyWarning { pub name : String , pub diag : Diagnostic , pub krate : String , # [doc = " The URL that points to the file and line of the lint emission"] pub url : String , }
};
}
