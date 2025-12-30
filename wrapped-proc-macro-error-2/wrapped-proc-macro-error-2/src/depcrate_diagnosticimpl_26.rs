// Generated macro for impl_26 (impl)
macro_rules! Depcrate_diagnosticimpl_26 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_26"}
// Dependencies: {}
# [doc = " **NOT PUBLIC API! NOTHING TO SEE HERE!!!**"] # [doc (hidden)] impl Diagnostic { pub fn span_suggestion (self , span : Span , suggestion : & str , msg : String) -> Self { match suggestion { "help" | "hint" => self . span_help (span , msg) , _ => self . span_note (span , msg) , } } pub fn suggestion (self , suggestion : & str , msg : String) -> Self { match suggestion { "help" | "hint" => self . help (msg) , _ => self . note (msg) , } } }
};
}
