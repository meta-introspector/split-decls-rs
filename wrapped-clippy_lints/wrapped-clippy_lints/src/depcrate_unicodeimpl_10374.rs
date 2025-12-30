// Generated macro for impl_10374 (impl)
macro_rules! Depcrate_unicodeimpl_10374 {
() => {
// Module: crate::unicode
// Provides: {"impl_10374"}
// Dependencies: {}
impl LateLintPass < '_ > for Unicode { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if let ExprKind :: Lit (lit) = expr . kind && let LitKind :: Str (_ , _) | LitKind :: Char (_) = lit . node { check_str (cx , lit . span , expr . hir_id) ; } } }
};
}
