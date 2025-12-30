// Generated macro for check (function)
macro_rules! Depcrate_methods_needless_option_as_derefcheck {
() => {
// Module: crate::methods::needless_option_as_deref
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , name : Symbol) { let typeck = cx . typeck_results () ; let outer_ty = typeck . expr_ty (expr) ; if outer_ty . is_diag_item (cx , sym :: Option) && outer_ty == typeck . expr_ty (recv) { if name == sym :: as_deref_mut && recv . is_syntactic_place_expr () { let Res :: Local (binding_id) = * recv . basic_res () else { return ; } ; if local_used_after_expr (cx , binding_id , recv) { return ; } } span_lint_and_sugg (cx , NEEDLESS_OPTION_AS_DEREF , expr . span , "derefed type is same as origin" , "try" , recv . span . get_source_text (cx) . unwrap () . to_owned () , Applicability :: MachineApplicable ,) ; } }
};
}
