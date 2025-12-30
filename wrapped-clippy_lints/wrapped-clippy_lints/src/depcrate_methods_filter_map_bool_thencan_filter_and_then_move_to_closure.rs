// Generated macro for can_filter_and_then_move_to_closure (function)
macro_rules! Depcrate_methods_filter_map_bool_thencan_filter_and_then_move_to_closure {
() => {
// Module: crate::methods::filter_map_bool_then
// Provides: {"can_filter_and_then_move_to_closure"}
// Dependencies: {}
# [doc = " Returns true if we can take a closure parameter and have it in both the `filter` function and"] # [doc = " the`map` function. This is not the case if:"] # [doc = ""] # [doc = " - The `filter` would contain an early return,"] # [doc = " - `filter` and `then` contain captures, and any of those are &mut"] fn can_filter_and_then_move_to_closure < 'tcx > (cx : & LateContext < 'tcx > , param : & Param < 'tcx > , filter : & 'tcx Expr < 'tcx > , then : & 'tcx Expr < 'tcx > ,) -> bool { if contains_return (filter) { return false ; } let Some (filter_captures) = can_move_expr_to_closure (cx , filter) else { return true ; } ; let Some (then_captures) = can_move_expr_to_closure (cx , then) else { return true ; } ; let param_bindings = find_bindings_from_pat (param . pat) ; filter_captures . iter () . all (| (hir_id , filter_cap) | { param_bindings . contains (hir_id) || ! then_captures . get (hir_id) . is_some_and (| then_cap | matches ! (* filter_cap | * then_cap , CaptureKind :: Ref (Mutability :: Mut))) }) }
};
}
