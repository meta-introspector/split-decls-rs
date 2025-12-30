// Generated macro for impl_9590 (impl)
macro_rules! Depcrate_size_of_refimpl_9590 {
() => {
// Module: crate::size_of_ref
// Provides: {"impl_9590"}
// Dependencies: {}
impl LateLintPass < '_ > for SizeOfRef { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if let ExprKind :: Call (path , [arg]) = expr . kind && let Some (def_id) = path_def_id (cx , path) && cx . tcx . is_diagnostic_item (sym :: mem_size_of_val , def_id) && let arg_ty = cx . typeck_results () . expr_ty (arg) && peel_middle_ty_refs (arg_ty) . 1 > 1 { span_lint_and_help (cx , SIZE_OF_REF , expr . span , "argument to `size_of_val()` is a reference to a reference" , None , "dereference the argument to `size_of_val()` to get the size of the value instead of the size of the reference-type" ,) ; } } }
};
}
