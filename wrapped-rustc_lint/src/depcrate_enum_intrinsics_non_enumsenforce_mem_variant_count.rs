// Generated macro for enforce_mem_variant_count (function)
macro_rules! Depcrate_enum_intrinsics_non_enumsenforce_mem_variant_count {
() => {
// Module: crate::enum_intrinsics_non_enums
// Provides: {"enforce_mem_variant_count"}
// Dependencies: {}
fn enforce_mem_variant_count (cx : & LateContext < '_ > , func_expr : & hir :: Expr < '_ > , span : Span) { let ty_param = cx . typeck_results () . node_args (func_expr . hir_id) . type_at (0) ; if is_non_enum (ty_param) { cx . emit_span_lint (ENUM_INTRINSICS_NON_ENUMS , span , EnumIntrinsicsMemVariant { ty_param }) ; } }
};
}
