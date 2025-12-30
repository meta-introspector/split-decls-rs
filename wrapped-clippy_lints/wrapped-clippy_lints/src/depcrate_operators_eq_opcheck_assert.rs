// Generated macro for check_assert (function)
macro_rules! Depcrate_operators_eq_opcheck_assert {
() => {
// Module: crate::operators::eq_op
// Provides: {"check_assert"}
// Dependencies: {}
pub (crate) fn check_assert < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if let Some (macro_call) = first_node_macro_backtrace (cx , e) . find (| macro_call | { matches ! (cx . tcx . get_diagnostic_name (macro_call . def_id) , Some (sym :: assert_eq_macro | sym :: assert_ne_macro | sym :: debug_assert_eq_macro | sym :: debug_assert_ne_macro)) }) && let Some ((lhs , rhs , _)) = find_assert_eq_args (cx , e , macro_call . expn) && eq_expr_value (cx , lhs , rhs) && macro_call . is_local () && ! is_in_test_function (cx . tcx , e . hir_id) { span_lint (cx , EQ_OP , lhs . span . to (rhs . span) , format ! ("identical args used in this `{}!` macro call" , cx . tcx . item_name (macro_call . def_id)) ,) ; } }
};
}
