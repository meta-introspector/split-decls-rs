// Generated macro for impl_264 (impl)
macro_rules! Depcrate_higherimpl_264 {
() => {
// Module: crate::higher
// Provides: {"impl_264"}
// Dependencies: {}
impl < 'hir > If < 'hir > { # [inline] # [doc = " Parses an `if` expression without `let`"] pub const fn hir (expr : & Expr < 'hir >) -> Option < Self > { if let ExprKind :: If (cond , then , r#else) = expr . kind && ! has_let_expr (cond) { Some (Self { cond , then , r#else }) } else { None } } }
};
}
