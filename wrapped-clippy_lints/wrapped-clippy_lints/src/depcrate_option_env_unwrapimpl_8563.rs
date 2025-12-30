// Generated macro for impl_8563 (impl)
macro_rules! Depcrate_option_env_unwrapimpl_8563 {
() => {
// Module: crate::option_env_unwrap
// Provides: {"impl_8563"}
// Dependencies: {}
impl EarlyLintPass for OptionEnvUnwrap { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: MethodCall (box MethodCall { seg , receiver , .. }) = & expr . kind && matches ! (seg . ident . name , sym :: expect | sym :: unwrap) && is_direct_expn_of (receiver . span , sym :: option_env) . is_some () { span_lint_and_help (cx , OPTION_ENV_UNWRAP , expr . span , "this will panic at run-time if the environment variable doesn't exist at compile-time" , None , "consider using the `env!` macro instead" ,) ; } } }
};
}
