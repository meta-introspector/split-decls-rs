// Generated macro for impl_278 (impl)
macro_rules! Depcrate_higherimpl_278 {
() => {
// Module: crate::higher
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'hir > WhileLet < 'hir > { # [inline] # [doc = " Parses a desugared `while let` loop"] pub const fn hir (expr : & Expr < 'hir >) -> Option < Self > { if let ExprKind :: Loop (& Block { expr : Some (& Expr { kind : ExprKind :: If (& Expr { kind : ExprKind :: Let (& hir :: LetExpr { pat : let_pat , init : let_expr , span : let_span , .. }) , .. } , if_then , _ ,) , .. }) , .. } , label , LoopSource :: While , _ ,) = expr . kind { return Some (Self { let_pat , let_expr , if_then , label , let_span , }) ; } None } }
};
}
