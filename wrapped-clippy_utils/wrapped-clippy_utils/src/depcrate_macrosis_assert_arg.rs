// Generated macro for is_assert_arg (function)
macro_rules! Depcrate_macrosis_assert_arg {
() => {
// Module: crate::macros
// Provides: {"is_assert_arg"}
// Dependencies: {}
fn is_assert_arg (cx : & LateContext < '_ > , expr : & Expr < '_ > , assert_expn : ExpnId) -> bool { if ! expr . span . from_expansion () { return true ; } let result = macro_backtrace (expr . span) . try_for_each (| macro_call | { if macro_call . expn == assert_expn { ControlFlow :: Break (false) } else { match cx . tcx . item_name (macro_call . def_id) { sym :: cfg => ControlFlow :: Continue (()) , _ => ControlFlow :: Break (true) , } } }) ; match result { ControlFlow :: Break (is_assert_arg) => is_assert_arg , ControlFlow :: Continue (()) => true , } }
};
}
