// Generated macro for Diagnostic (struct)
macro_rules! Depcrate_executor_jsonDiagnostic {
() => {
// Module: crate::executor::json
// Provides: {"Diagnostic"}
// Dependencies: {}
# [derive (Deserialize)] struct Diagnostic { message : String , code : Option < DiagnosticCode > , level : String , spans : Vec < DiagnosticSpan > , children : Vec < Diagnostic > , rendered : Option < String > , }
};
}
