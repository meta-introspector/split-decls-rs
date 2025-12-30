// Generated macro for can_replace_with_contains (function)
macro_rules! Depcrate_methods_manual_containscan_replace_with_contains {
() => {
// Module: crate::methods::manual_contains
// Provides: {"can_replace_with_contains"}
// Dependencies: {}
fn can_replace_with_contains < 'tcx > (cx : & LateContext < 'tcx > , bin_op : Spanned < BinOpKind > , left_expr : & 'tcx Expr < 'tcx > , right_expr : & 'tcx Expr < 'tcx > , closure_arg_id : HirId , applicability : & mut Applicability ,) -> Option < (String , & 'tcx Expr < 'tcx >) > { if bin_op . node != BinOpKind :: Eq { return None ; } let left_candidate = try_get_eligible_arg (cx , left_expr , closure_arg_id , applicability) ? ; let right_candidate = try_get_eligible_arg (cx , right_expr , closure_arg_id , applicability) ? ; match (left_candidate , right_candidate) { ((EligibleArg :: IsClosureArg , _) , (EligibleArg :: ContainsArg (snip) , candidate_expr)) | ((EligibleArg :: ContainsArg (snip) , candidate_expr) , (EligibleArg :: IsClosureArg , _)) => { Some ((snip , candidate_expr)) } , _ => None , } }
};
}
