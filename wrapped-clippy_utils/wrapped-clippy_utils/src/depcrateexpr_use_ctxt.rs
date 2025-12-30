// Generated macro for expr_use_ctxt (function)
macro_rules! Depcrateexpr_use_ctxt {
() => {
// Module: crate
// Provides: {"expr_use_ctxt"}
// Dependencies: {}
# [doc = " Gets the context an expression's value is used in."] pub fn expr_use_ctxt < 'tcx > (cx : & LateContext < 'tcx > , e : & Expr < 'tcx >) -> ExprUseCtxt < 'tcx > { let mut adjustments = [] . as_slice () ; let mut is_ty_unified = false ; let mut moved_before_use = false ; let mut same_ctxt = true ; let ctxt = e . span . ctxt () ; let node = walk_to_expr_usage (cx , e , & mut | parent_id , parent , child_id | -> ControlFlow < ! > { if adjustments . is_empty () && let Node :: Expr (e) = cx . tcx . hir_node (child_id) { adjustments = cx . typeck_results () . expr_adjustments (e) ; } same_ctxt &= cx . tcx . hir_span (parent_id) . ctxt () == ctxt ; if let Node :: Expr (e) = parent { match e . kind { ExprKind :: If (e , _ , _) | ExprKind :: Match (e , _ , _) if e . hir_id != child_id => { is_ty_unified = true ; moved_before_use = true ; } , ExprKind :: Block (_ , Some (_)) | ExprKind :: Break (..) => { is_ty_unified = true ; moved_before_use = true ; } , ExprKind :: Block (..) => moved_before_use = true , _ => { } , } } ControlFlow :: Continue (()) }) ; match node { Some (ControlFlow :: Continue ((node , child_id))) => ExprUseCtxt { node , child_id , adjustments , is_ty_unified , moved_before_use , same_ctxt , } , Some (ControlFlow :: Break (_)) => unreachable ! ("type of node is ControlFlow<!>") , None => ExprUseCtxt { node : Node :: Crate (cx . tcx . hir_root_module ()) , child_id : HirId :: INVALID , adjustments : & [] , is_ty_unified : true , moved_before_use : true , same_ctxt : false , } , } }
};
}
