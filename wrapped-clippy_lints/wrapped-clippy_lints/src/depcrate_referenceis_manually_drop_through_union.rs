// Generated macro for is_manually_drop_through_union (function)
macro_rules! Depcrate_referenceis_manually_drop_through_union {
() => {
// Module: crate::reference
// Provides: {"is_manually_drop_through_union"}
// Dependencies: {}
# [doc = " Check if `addrof_target` is part of an access to a `ManuallyDrop` entity reached through a"] # [doc = " union, and when it is dereferenced using `DerefMut` starting from `expr_id` and going up."] fn is_manually_drop_through_union (cx : & LateContext < '_ > , expr_id : HirId , addrof_target : & Expr < '_ > ,) -> ManuallyDropThroughUnion { if is_reached_through_union (cx , addrof_target) { let typeck = cx . typeck_results () ; for (idx , id) in std :: iter :: once (expr_id) . chain (cx . tcx . hir_parent_id_iter (expr_id)) . enumerate () { if let Node :: Expr (expr) = cx . tcx . hir_node (id) { if adjust_derefs_manually_drop (typeck . expr_adjustments (expr) , typeck . expr_ty (expr)) { return if idx == 0 { ManuallyDropThroughUnion :: Directly } else { ManuallyDropThroughUnion :: Indirect } ; } } else { break ; } } } ManuallyDropThroughUnion :: No }
};
}
