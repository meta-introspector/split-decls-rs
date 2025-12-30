// Generated macro for impl_278 (impl)
macro_rules! Depcrate_error_childimpl_278 {
() => {
// Module: crate::error::child
// Provides: {"impl_278"}
// Dependencies: {}
impl ChildDiagnostic { # [doc = " Append this child diagnostic to a `Diagnostic`."] # [doc = ""] # [doc = " # Panics"] # [doc = " This method panics if `self` has a span and is being invoked outside of"] # [doc = " a proc-macro due to the behavior of [`Span::unwrap()`](Span)."] pub fn append_to (self , diagnostic : proc_macro :: Diagnostic) -> proc_macro :: Diagnostic { match self . level { Level :: Error => { if let Some (span) = self . span { diagnostic . span_error (span . unwrap () , self . message) } else { diagnostic . error (self . message) } } Level :: Warning => { if let Some (span) = self . span { diagnostic . span_warning (span . unwrap () , self . message) } else { diagnostic . warning (self . message) } } Level :: Note => { if let Some (span) = self . span { diagnostic . span_note (span . unwrap () , self . message) } else { diagnostic . note (self . message) } } Level :: Help => { if let Some (span) = self . span { diagnostic . span_help (span . unwrap () , self . message) } else { diagnostic . help (self . message) } } } } }
};
}
