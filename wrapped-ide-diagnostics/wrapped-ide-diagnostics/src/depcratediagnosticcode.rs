// Generated macro for DiagnosticCode (enum)
macro_rules! DepcrateDiagnosticCode {
() => {
// Module: crate
// Provides: {"DiagnosticCode"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum DiagnosticCode { RustcHardError (& 'static str) , SyntaxError , RustcLint (& 'static str) , Clippy (& 'static str) , Ra (& 'static str , Severity) , }
};
}
