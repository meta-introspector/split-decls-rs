// Generated macro for impl_8740 (impl)
macro_rules! Depcrate_permissions_set_readonly_falseimpl_8740 {
() => {
// Module: crate::permissions_set_readonly_false
// Provides: {"impl_8740"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PermissionsSetReadonlyFalse { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: MethodCall (path , receiver , [arg] , _) = & expr . kind && let ExprKind :: Lit (lit) = & arg . kind && LitKind :: Bool (false) == lit . node && path . ident . name == sym :: set_readonly && is_type_diagnostic_item (cx , cx . typeck_results () . expr_ty (receiver) , sym :: FsPermissions) { span_lint_and_then (cx , PERMISSIONS_SET_READONLY_FALSE , expr . span , "call to `set_readonly` with argument `false`" , | diag | { diag . note ("on Unix platforms this results in the file being world writable") ; diag . help ("you can set the desired permissions using `PermissionsExt`. For more information, see\n\
                        https://doc.rust-lang.org/std/os/unix/fs/trait.PermissionsExt.html" ,) ; } ,) ; } } }
};
}
