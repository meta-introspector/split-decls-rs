// Generated macro for impl_327 (impl)
macro_rules! Depcrate_enum_intrinsics_non_enumsimpl_327 {
() => {
// Module: crate::enum_intrinsics_non_enums
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for EnumIntrinsicsNonEnums { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) { let hir :: ExprKind :: Call (func , args) = & expr . kind else { return } ; let hir :: ExprKind :: Path (qpath) = & func . kind else { return } ; let Some (def_id) = cx . qpath_res (qpath , func . hir_id) . opt_def_id () else { return } ; let Some (name) = cx . tcx . get_diagnostic_name (def_id) else { return } ; match name { sym :: mem_discriminant => enforce_mem_discriminant (cx , func , expr . span , args [0] . span) , sym :: mem_variant_count => enforce_mem_variant_count (cx , func , expr . span) , _ => { } } } }
};
}
