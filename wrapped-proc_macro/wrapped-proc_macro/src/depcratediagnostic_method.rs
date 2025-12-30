// Generated macro for diagnostic_method (macro)
macro_rules! Depcratediagnostic_method {
() => {
// Module: crate
// Provides: {"diagnostic_method"}
// Dependencies: {}
macro_rules ! diagnostic_method { ($ name : ident , $ level : expr) => { # [doc = " Creates a new `Diagnostic` with the given `message` at the span"] # [doc = " `self`."] # [unstable (feature = "proc_macro_diagnostic" , issue = "54140")] pub fn $ name < T : Into < String >> (self , message : T) -> Diagnostic { Diagnostic :: spanned (self , $ level , message) } } ; }
};
}
