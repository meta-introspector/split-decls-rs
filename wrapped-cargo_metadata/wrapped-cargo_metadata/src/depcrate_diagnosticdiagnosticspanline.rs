// Generated macro for DiagnosticSpanLine (struct)
macro_rules! Depcrate_diagnosticDiagnosticSpanLine {
() => {
// Module: crate::diagnostic
// Provides: {"DiagnosticSpanLine"}
// Dependencies: {}
# [doc = " A line of code associated with the Diagnostic"] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct DiagnosticSpanLine { # [doc = " The line of code associated with the error"] pub text : String , # [doc = " Start of the section of the line to highlight. 1-based, character offset in self.text"] pub highlight_start : usize , # [doc = " End of the section of the line to highlight. 1-based, character offset in self.text"] pub highlight_end : usize , }
};
}
