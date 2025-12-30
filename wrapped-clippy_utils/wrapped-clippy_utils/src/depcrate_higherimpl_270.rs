// Generated macro for impl_270 (impl)
macro_rules! Depcrate_higherimpl_270 {
() => {
// Module: crate::higher
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'hir > IfOrIfLet < 'hir > { # [inline] # [doc = " Parses an `if` or `if let` expression"] pub const fn hir (expr : & Expr < 'hir >) -> Option < Self > { if let ExprKind :: If (cond , then , r#else) = expr . kind { Some (Self { cond , then , r#else }) } else { None } } }
};
}
