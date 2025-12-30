// Generated macro for is_used_as_unaligned (function)
macro_rules! Depcrate_casts_cast_ptr_alignmentis_used_as_unaligned {
() => {
// Module: crate::casts::cast_ptr_alignment
// Provides: {"is_used_as_unaligned"}
// Dependencies: {}
fn is_used_as_unaligned (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { let Some (parent) = get_parent_expr (cx , e) else { return false ; } ; match parent . kind { ExprKind :: MethodCall (name , self_arg , ..) if self_arg . hir_id == e . hir_id => { if matches ! (name . ident . name , sym :: read_unaligned | sym :: write_unaligned) && let Some (def_id) = cx . typeck_results () . type_dependent_def_id (parent . hir_id) && let Some (def_id) = cx . tcx . impl_of_assoc (def_id) && cx . tcx . type_of (def_id) . instantiate_identity () . is_raw_ptr () { true } else { false } } , ExprKind :: Call (func , [arg , ..]) if arg . hir_id == e . hir_id => { if let ExprKind :: Path (path) = & func . kind && let Some (def_id) = cx . qpath_res (path , func . hir_id) . opt_def_id () && let Some (name) = cx . tcx . get_diagnostic_name (def_id) && matches ! (name , sym :: ptr_write_unaligned | sym :: ptr_read_unaligned | sym :: intrinsics_unaligned_volatile_load | sym :: intrinsics_unaligned_volatile_store) { true } else { false } } , _ => false , } }
};
}
