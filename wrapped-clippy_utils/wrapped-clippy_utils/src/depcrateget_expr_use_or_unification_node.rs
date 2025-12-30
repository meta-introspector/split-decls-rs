// Generated macro for get_expr_use_or_unification_node (function)
macro_rules! Depcrateget_expr_use_or_unification_node {
() => {
// Module: crate
// Provides: {"get_expr_use_or_unification_node"}
// Dependencies: {}
# [doc = " Gets the node where an expression is either used, or it's type is unified with another branch."] # [doc = " Returns both the node and the `HirId` of the closest child node."] pub fn get_expr_use_or_unification_node < 'tcx > (tcx : TyCtxt < 'tcx > , expr : & Expr < '_ >) -> Option < (Node < 'tcx > , HirId) > { let mut child_id = expr . hir_id ; let mut iter = tcx . hir_parent_iter (child_id) ; loop { match iter . next () { None => break None , Some ((id , Node :: Block (_))) => child_id = id , Some ((id , Node :: Arm (arm))) if arm . body . hir_id == child_id => child_id = id , Some ((_ , Node :: Expr (expr))) => match expr . kind { ExprKind :: Match (_ , [arm] , _) if arm . hir_id == child_id => child_id = expr . hir_id , ExprKind :: Block (..) | ExprKind :: DropTemps (_) => child_id = expr . hir_id , ExprKind :: If (_ , then_expr , None) if then_expr . hir_id == child_id => break None , _ => break Some ((Node :: Expr (expr) , child_id)) , } , Some ((_ , node)) => break Some ((node , child_id)) , } } }
};
}
