// Generated macro for impl_7688 (impl)
macro_rules! Depcrate_needless_continueimpl_7688 {
() => {
// Module: crate::needless_continue
// Provides: {"impl_7688"}
// Dependencies: {}
impl EarlyLintPass for NeedlessContinue { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & ast :: Expr) { if ! expr . span . from_expansion () { check_and_warn (cx , expr) ; } } }
};
}
