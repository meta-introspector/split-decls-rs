// Generated macro for impl_277 (impl)
macro_rules! Depcrate_higherimpl_277 {
() => {
// Module: crate::higher
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'hir > While < 'hir > { # [inline] # [doc = " Parses a desugared `while` loop"] pub const fn hir (expr : & Expr < 'hir >) -> Option < Self > { if let ExprKind :: Loop (Block { expr : Some (Expr { kind : ExprKind :: If (condition , body , _) , .. }) , .. } , _ , LoopSource :: While , span ,) = expr . kind && ! has_let_expr (condition) { return Some (Self { condition , body , span }) ; } None } }
};
}
