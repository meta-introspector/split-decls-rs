// Generated macro for impl_263 (impl)
macro_rules! Depcrate_higherimpl_263 {
() => {
// Module: crate::higher
// Provides: {"impl_263"}
// Dependencies: {}
impl < 'tcx > ForLoop < 'tcx > { # [doc = " Parses a desugared `for` loop"] pub fn hir (expr : & Expr < 'tcx >) -> Option < Self > { if let ExprKind :: DropTemps (e) = expr . kind && let ExprKind :: Match (iterexpr , [arm] , MatchSource :: ForLoopDesugar) = e . kind && let ExprKind :: Call (_ , [arg]) = iterexpr . kind && let ExprKind :: Loop (block , label , ..) = arm . body . kind && let [stmt] = block . stmts && let hir :: StmtKind :: Expr (e) = stmt . kind && let ExprKind :: Match (_ , [_ , some_arm] , _) = e . kind && let hir :: PatKind :: Struct (_ , [field] , _) = some_arm . pat . kind { return Some (Self { pat : field . pat , arg , body : some_arm . body , loop_id : arm . body . hir_id , span : expr . span . ctxt () . outer_expn_data () . call_site , label , }) ; } None } }
};
}
