// Generated macro for impl_267 (impl)
macro_rules! Depcrate_higherimpl_267 {
() => {
// Module: crate::higher
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'hir > IfLet < 'hir > { # [doc = " Parses an `if let` expression"] pub fn hir (cx : & LateContext < '_ > , expr : & Expr < 'hir >) -> Option < Self > { if let ExprKind :: If (& Expr { kind : ExprKind :: Let (& hir :: LetExpr { pat : let_pat , init : let_expr , span : let_span , .. }) , .. } , if_then , if_else ,) = expr . kind { let mut iter = cx . tcx . hir_parent_iter (expr . hir_id) ; if let Some ((_ , Node :: Block (Block { stmts : [] , .. }))) = iter . next () && let Some ((_ , Node :: Expr (Expr { kind : ExprKind :: Loop (_ , _ , LoopSource :: While , _) , .. }) ,)) = iter . next () { return None ; } return Some (Self { let_pat , let_expr , if_then , if_else , let_span , }) ; } None } }
};
}
