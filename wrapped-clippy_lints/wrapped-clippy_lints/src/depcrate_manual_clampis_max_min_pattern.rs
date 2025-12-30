// Generated macro for is_max_min_pattern (function)
macro_rules! Depcrate_manual_clampis_max_min_pattern {
() => {
// Module: crate::manual_clamp
// Provides: {"is_max_min_pattern"}
// Dependencies: {}
# [doc = " Targets patterns like"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let (input, min_value, max_value) = (0, -3, 12);"] # [doc = ""] # [doc = " input.max(min_value).min(max_value)"] # [doc = " # ;"] # [doc = " ```"] fn is_max_min_pattern < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < ClampSuggestion < 'tcx > > { if let ExprKind :: MethodCall (seg_second , receiver , [arg_second] , _) = expr . kind && (cx . typeck_results () . expr_ty_adjusted (receiver) . is_floating_point () || cx . ty_based_def (expr) . assoc_fn_parent (cx) . is_diag_item (cx , sym :: Ord)) && let ExprKind :: MethodCall (seg_first , input , [arg_first] , _) = & receiver . kind && (cx . typeck_results () . expr_ty_adjusted (input) . is_floating_point () || cx . ty_based_def (receiver) . assoc_fn_parent (cx) . is_diag_item (cx , sym :: Ord)) { let is_float = cx . typeck_results () . expr_ty_adjusted (input) . is_floating_point () ; let (min , max) = match (seg_first . ident . name , seg_second . ident . name) { (sym :: min , sym :: max) => (arg_second , arg_first) , (sym :: max , sym :: min) => (arg_first , arg_second) , _ => return None , } ; Some (ClampSuggestion { params : InputMinMax { input , min , max , is_float , } , span : expr . span , make_assignment : None , hir_with_ignore_attr : None , }) } else { None } }
};
}
