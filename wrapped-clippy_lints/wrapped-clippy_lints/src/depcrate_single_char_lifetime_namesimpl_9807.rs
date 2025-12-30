// Generated macro for impl_9807 (impl)
macro_rules! Depcrate_single_char_lifetime_namesimpl_9807 {
() => {
// Module: crate::single_char_lifetime_names
// Provides: {"impl_9807"}
// Dependencies: {}
impl EarlyLintPass for SingleCharLifetimeNames { fn check_generic_param (& mut self , cx : & EarlyContext < '_ > , param : & GenericParam) { if param . ident . span . in_external_macro (cx . sess () . source_map ()) { return ; } if let GenericParamKind :: Lifetime = param . kind && ! param . is_placeholder && param . ident . as_str () . len () <= 2 { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , SINGLE_CHAR_LIFETIME_NAMES , param . ident . span , "single-character lifetime names are likely uninformative" , | diag | { diag . help ("use a more informative name") ; } ,) ; } } }
};
}
