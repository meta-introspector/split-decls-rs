// Generated macro for DiagnosticLevel (enum)
macro_rules! Depcrate_format_diagnosticDiagnosticLevel {
() => {
// Module: crate::format::diagnostic
// Provides: {"DiagnosticLevel"}
// Dependencies: {}
# [doc = " The diagnostic level"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Serialize , Deserialize)] # [serde (rename_all = "lowercase")] pub enum DiagnosticLevel { # [doc = " Internal compiler error"] # [serde (rename = "error: internal compiler error")] Ice , # [doc = " Error"] Error , # [doc = " Warning"] Warning , # [doc = " Note"] Note , # [doc = " Help"] Help , # [cfg (not (feature = "strict_unstable"))] # [doc (hidden)] # [serde (other)] Unknown , }
};
}
