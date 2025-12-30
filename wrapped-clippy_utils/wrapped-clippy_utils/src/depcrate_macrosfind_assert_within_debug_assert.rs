// Generated macro for find_assert_within_debug_assert (function)
macro_rules! Depcrate_macrosfind_assert_within_debug_assert {
() => {
// Module: crate::macros
// Provides: {"find_assert_within_debug_assert"}
// Dependencies: {}
fn find_assert_within_debug_assert < 'a > (cx : & LateContext < '_ > , expr : & 'a Expr < 'a > , expn : ExpnId , assert_name : Symbol ,) -> Option < (& 'a Expr < 'a > , ExpnId) > { for_each_expr_without_closures (expr , | e | { if ! e . span . from_expansion () { return ControlFlow :: Continue (Descend :: No) ; } let e_expn = e . span . ctxt () . outer_expn () ; if e_expn == expn { ControlFlow :: Continue (Descend :: Yes) } else if e_expn . expn_data () . macro_def_id . map (| id | cx . tcx . item_name (id)) == Some (assert_name) { ControlFlow :: Break ((e , e_expn)) } else { ControlFlow :: Continue (Descend :: No) } }) }
};
}
