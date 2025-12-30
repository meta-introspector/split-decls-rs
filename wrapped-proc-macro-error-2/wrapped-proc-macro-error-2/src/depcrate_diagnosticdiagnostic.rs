// Generated macro for Diagnostic (struct)
macro_rules! Depcrate_diagnosticDiagnostic {
() => {
// Module: crate::diagnostic
// Provides: {"Diagnostic"}
// Dependencies: {}
# [doc = " Represents a single diagnostic message"] # [derive (Debug)] # [must_use = "A diagnostic does nothing unless emitted"] pub struct Diagnostic { pub (crate) level : Level , pub (crate) span_range : SpanRange , pub (crate) msg : String , pub (crate) suggestions : Vec < (SuggestionKind , String , Option < SpanRange >) > , pub (crate) children : Vec < (SpanRange , String) > , }
};
}
