// Generated macro for MultiSpan (trait)
macro_rules! Depcrate_diagnosticMultiSpan {
() => {
// Module: crate::diagnostic
// Provides: {"MultiSpan"}
// Dependencies: {}
# [doc = " Trait implemented by types that can be converted into a set of `Span`s."] # [unstable (feature = "proc_macro_diagnostic" , issue = "54140")] pub trait MultiSpan { # [doc = " Converts `self` into a `Vec<Span>`."] fn into_spans (self) -> Vec < Span > ; }
};
}
