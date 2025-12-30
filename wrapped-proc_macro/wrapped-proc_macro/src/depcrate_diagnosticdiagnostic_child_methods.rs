// Generated macro for diagnostic_child_methods (macro)
macro_rules! Depcrate_diagnosticdiagnostic_child_methods {
() => {
// Module: crate::diagnostic
// Provides: {"diagnostic_child_methods"}
// Dependencies: {}
macro_rules ! diagnostic_child_methods { ($ spanned : ident , $ regular : ident , $ level : expr) => { # [unstable (feature = "proc_macro_diagnostic" , issue = "54140")] # [doc = concat ! ("Adds a new child diagnostics message to `self` with the [`" , stringify ! ($ level) , "`] level, and the given `spans` and `message`.")] pub fn $ spanned < S , T > (mut self , spans : S , message : T) -> Diagnostic where S : MultiSpan , T : Into < String >, { self . children . push (Diagnostic :: spanned (spans , $ level , message)) ; self } # [unstable (feature = "proc_macro_diagnostic" , issue = "54140")] # [doc = concat ! ("Adds a new child diagnostic message to `self` with the [`" , stringify ! ($ level) , "`] level, and the given `message`.")] pub fn $ regular < T : Into < String >> (mut self , message : T) -> Diagnostic { self . children . push (Diagnostic :: new ($ level , message)) ; self } } ; }
};
}
