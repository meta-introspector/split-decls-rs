// Generated macro for impl_9580 (impl)
macro_rules! Depcrate_size_of_in_element_countimpl_9580 {
() => {
// Module: crate::size_of_in_element_count
// Provides: {"impl_9580"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SizeOfInElementCount { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { const HELP_MSG : & str = "use a count of elements instead of a count of bytes\
            , it already gets multiplied by the size of the type" ; const LINT_MSG : & str = "found a count of bytes \
             instead of a count of elements of `T`" ; if let Some ((pointee_ty , count_expr)) = get_pointee_ty_and_count_expr (cx , expr) && pointee_ty != cx . tcx . types . u8 && let Some (ty_used_for_size_of) = get_size_of_ty (cx , count_expr , false) && pointee_ty == ty_used_for_size_of { span_lint_and_help (cx , SIZE_OF_IN_ELEMENT_COUNT , count_expr . span , LINT_MSG , None , HELP_MSG) ; } } }
};
}
