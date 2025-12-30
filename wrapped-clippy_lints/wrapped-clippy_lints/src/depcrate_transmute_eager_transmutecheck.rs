// Generated macro for check (function)
macro_rules! Depcrate_transmute_eager_transmutecheck {
() => {
// Module: crate::transmute::eager_transmute
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , transmutable : & 'tcx Expr < 'tcx > , from_ty : Ty < 'tcx > , to_ty : Ty < 'tcx > ,) -> bool { if let Some (then_some_call) = peel_parent_unsafe_blocks (cx , expr) && let ExprKind :: MethodCall (path , receiver , [arg] , _) = then_some_call . kind && cx . typeck_results () . expr_ty (receiver) . is_bool () && path . ident . name == sym :: then_some && is_local_with_projections (transmutable) && binops_with_local (cx , transmutable , receiver) && let Ok (from_layout) = cx . tcx . layout_of (cx . typing_env () . as_query_input (from_ty)) && let Ok (to_layout) = cx . tcx . layout_of (cx . typing_env () . as_query_input (to_ty)) && match (from_layout . largest_niche , to_layout . largest_niche) { (Some (from_niche) , Some (to_niche)) => ! range_fully_contained (from_niche . valid_range , to_niche . valid_range) , (None , Some (_)) => true , (_ , None) => false , } { span_lint_and_then (cx , EAGER_TRANSMUTE , expr . span , "this transmute is always evaluated eagerly, even if the condition is false" , | diag | { diag . multipart_suggestion ("consider using `bool::then` to only transmute if the condition holds" , vec ! [(path . ident . span , "then" . into ()) , (arg . span . shrink_to_lo () , "|| " . into ()) ,] , Applicability :: MaybeIncorrect ,) ; } ,) ; true } else { false } }
};
}
