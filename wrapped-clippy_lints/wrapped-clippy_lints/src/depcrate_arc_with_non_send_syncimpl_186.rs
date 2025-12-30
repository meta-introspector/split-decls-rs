// Generated macro for impl_186 (impl)
macro_rules! Depcrate_arc_with_non_send_syncimpl_186 {
() => {
// Module: crate::arc_with_non_send_sync
// Provides: {"impl_186"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ArcWithNonSendSync { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: Call (func , [arg]) = expr . kind && let ExprKind :: Path (QPath :: TypeRelative (func_ty , func_name)) = func . kind && func_name . ident . name == sym :: new && ! expr . span . from_expansion () && cx . typeck_results () . node_type (func_ty . hir_id) . is_diag_item (cx , sym :: Arc) && let arg_ty = cx . typeck_results () . expr_ty (arg) && arg_ty . walk () . all (| arg | { ! matches ! (arg . kind () , GenericArgKind :: Type (ty) if matches ! (ty . kind () , ty :: Param (_))) }) && let Some (send) = cx . tcx . get_diagnostic_item (sym :: Send) && let Some (sync) = cx . tcx . lang_items () . sync_trait () && let [is_send , is_sync] = [send , sync] . map (| id | implements_trait (cx , arg_ty , id , & [])) && let reason = match (is_send , is_sync) { (false , false) => "neither `Send` nor `Sync`" , (false , true) => "not `Send`" , (true , false) => "not `Sync`" , _ => return , } && ! is_from_proc_macro (cx , expr) { span_lint_and_then (cx , ARC_WITH_NON_SEND_SYNC , expr . span , "usage of an `Arc` that is not `Send` and `Sync`" , | diag | { with_forced_trimmed_paths ! ({ diag . note (format ! ("`Arc<{arg_ty}>` is not `Send` and `Sync` as `{arg_ty}` is {reason}")) ; diag . help ("if the `Arc` will not be used across threads replace it with an `Rc`") ; diag . help (format ! ("otherwise make `{arg_ty}` `Send` and `Sync` or consider a wrapper type such as `Mutex`")) ; }) ; } ,) ; } } }
};
}
