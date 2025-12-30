// Generated macro for DiagnosticSpanLine (struct)
macro_rules! Depcrate_format_diagnosticDiagnosticSpanLine {
() => {
// Module: crate::format::diagnostic
// Provides: {"DiagnosticSpanLine"}
// Dependencies: {}
# [doc = " A line of code associated with the Diagnostic"] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct DiagnosticSpanLine < 'a > { # [doc = " The line of code associated with the error"] # [serde (borrow)] pub text : CowStr < 'a > , # [doc = " Start of the section of the line to highlight. 1-based, character offset in self.text"] pub highlight_start : usize , # [doc = " End of the section of the line to highlight. 1-based, character offset in self.text"] pub highlight_end : usize , }
};
}
