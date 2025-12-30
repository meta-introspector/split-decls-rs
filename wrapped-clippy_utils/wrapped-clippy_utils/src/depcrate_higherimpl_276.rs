// Generated macro for impl_276 (impl)
macro_rules! Depcrate_higherimpl_276 {
() => {
// Module: crate::higher
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'hir > While < 'hir > { # [inline] # [doc = " Parses a desugared `while` loop"] pub const fn hir (expr : & Expr < 'hir >) -> Option < Self > { if let ExprKind :: Loop (Block { expr : Some (Expr { kind : ExprKind :: If (condition , body , _) , .. }) , .. } , label , LoopSource :: While , span ,) = expr . kind && ! has_let_expr (condition) { return Some (Self { condition , body , span , label , }) ; } None } }
};
}
