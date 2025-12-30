// Generated macro for impl_4424 (impl)
macro_rules! Depcrate_manual_string_newimpl_4424 {
() => {
// Module: crate::manual_string_new
// Provides: {"impl_4424"}
// Dependencies: {}
impl LateLintPass < '_ > for ManualStringNew { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if expr . span . from_expansion () { return ; } let ty = cx . typeck_results () . expr_ty (expr) ; match ty . kind () { ty :: Adt (adt_def , _) if adt_def . is_struct () => { if cx . tcx . lang_items () . string () != Some (adt_def . did ()) { return ; } } , _ => return , } match expr . kind { ExprKind :: Call (func , [arg]) => { parse_call (cx , expr . span , func , arg) ; } , ExprKind :: MethodCall (path_segment , receiver , ..) => { parse_method_call (cx , expr . span , path_segment , receiver) ; } , _ => () , } } }
};
}
