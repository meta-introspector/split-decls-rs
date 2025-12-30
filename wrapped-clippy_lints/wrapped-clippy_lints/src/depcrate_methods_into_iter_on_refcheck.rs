// Generated macro for check (function)
macro_rules! Depcrate_methods_into_iter_on_refcheck {
() => {
// Module: crate::methods::into_iter_on_ref
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , method_span : Span , receiver : & hir :: Expr < '_ >) { let self_ty = cx . typeck_results () . expr_ty_adjusted (receiver) ; if let ty :: Ref (..) = self_ty . kind () && cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: IntoIterator) && let Some ((kind , method_name)) = ty_has_iter_method (cx , self_ty) { span_lint_and_sugg (cx , INTO_ITER_ON_REF , method_span , format ! ("this `.into_iter()` call is equivalent to `.{method_name}()` and will not consume the `{kind}`" ,) , "call directly" , method_name . to_string () , Applicability :: MachineApplicable ,) ; } }
};
}
