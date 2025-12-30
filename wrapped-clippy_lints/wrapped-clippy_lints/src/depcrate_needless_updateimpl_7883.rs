// Generated macro for impl_7883 (impl)
macro_rules! Depcrate_needless_updateimpl_7883 {
() => {
// Module: crate::needless_update
// Provides: {"impl_7883"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NeedlessUpdate { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Struct (_ , fields , StructTailExpr :: Base (base)) = expr . kind { let ty = cx . typeck_results () . expr_ty (expr) ; if let ty :: Adt (def , _) = ty . kind () && fields . len () == def . non_enum_variant () . fields . len () && ! def . variant (0_usize . into ()) . is_field_list_non_exhaustive () { span_lint (cx , NEEDLESS_UPDATE , base . span , "struct update has no effect, all the fields in the struct have already been specified" ,) ; } } } }
};
}
