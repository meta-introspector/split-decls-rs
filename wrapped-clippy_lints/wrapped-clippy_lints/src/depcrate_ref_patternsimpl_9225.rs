// Generated macro for impl_9225 (impl)
macro_rules! Depcrate_ref_patternsimpl_9225 {
() => {
// Module: crate::ref_patterns
// Provides: {"impl_9225"}
// Dependencies: {}
impl EarlyLintPass for RefPatterns { fn check_pat (& mut self , cx : & EarlyContext < '_ > , pat : & Pat) { if let PatKind :: Ident (BindingMode :: REF , _ , _) = pat . kind && ! pat . span . from_expansion () { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , REF_PATTERNS , pat . span , "usage of ref pattern" , | diag | { diag . help ("consider using `&` for clarity instead") ; }) ; } } }
};
}
