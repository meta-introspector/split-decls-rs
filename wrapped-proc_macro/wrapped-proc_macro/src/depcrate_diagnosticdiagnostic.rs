// Generated macro for Diagnostic (struct)
macro_rules! Depcrate_diagnosticDiagnostic {
() => {
// Module: crate::diagnostic
// Provides: {"Diagnostic"}
// Dependencies: {}
# [doc = " A structure representing a diagnostic message and associated children"] # [doc = " messages."] # [unstable (feature = "proc_macro_diagnostic" , issue = "54140")] # [derive (Clone , Debug)] pub struct Diagnostic { level : Level , message : String , spans : Vec < Span > , children : Vec < Diagnostic > , }
};
}
