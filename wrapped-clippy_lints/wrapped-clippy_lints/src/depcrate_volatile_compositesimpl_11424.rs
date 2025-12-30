// Generated macro for impl_11424 (impl)
macro_rules! Depcrate_volatile_compositesimpl_11424 {
() => {
// Module: crate::volatile_composites
// Provides: {"impl_11424"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for VolatileComposites { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { match expr . kind { ExprKind :: MethodCall (name , self_arg , _ , _) if matches ! (name . ident . name , sym :: read_volatile | sym :: write_volatile) => { let self_ty = cx . typeck_results () . expr_ty (self_arg) ; match self_ty . kind () { ty :: RawPtr (innerty , _) => report_volatile_safe (cx , expr , * innerty) , ty :: Adt (_ , args) if self_ty . is_diag_item (cx , sym :: NonNull) => { report_volatile_safe (cx , expr , args . type_at (0)) ; } , _ => () , } } , ExprKind :: Call (func , [arg_ptr , ..]) => { if let ExprKind :: Path (ref qpath) = func . kind && let Some (def_id) = cx . qpath_res (qpath , func . hir_id) . opt_def_id () && matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: ptr_read_volatile | sym :: ptr_write_volatile)) && let ty :: RawPtr (ptrty , _) = cx . typeck_results () . expr_ty_adjusted (arg_ptr) . kind () { report_volatile_safe (cx , expr , * ptrty) ; } } , _ => { } , } } }
};
}
