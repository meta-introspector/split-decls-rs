// Generated macro for find_assert_args_inner (function)
macro_rules! Depcrate_macrosfind_assert_args_inner {
() => {
// Module: crate::macros
// Provides: {"find_assert_args_inner"}
// Dependencies: {}
fn find_assert_args_inner < 'a , const N : usize > (cx : & LateContext < '_ > , expr : & 'a Expr < 'a > , expn : ExpnId ,) -> Option < ([& 'a Expr < 'a > ; N] , PanicExpn < 'a >) > { let macro_id = expn . expn_data () . macro_def_id ? ; let (expr , expn) = match cx . tcx . item_name (macro_id) . as_str () . strip_prefix ("debug_") { None => (expr , expn) , Some (inner_name) => find_assert_within_debug_assert (cx , expr , expn , Symbol :: intern (inner_name)) ? , } ; let mut args = ArrayVec :: new () ; let panic_expn = for_each_expr_without_closures (expr , | e | { if args . is_full () { match PanicExpn :: parse (e) { Some (expn) => ControlFlow :: Break (expn) , None => ControlFlow :: Continue (Descend :: Yes) , } } else if is_assert_arg (cx , e , expn) { args . push (e) ; ControlFlow :: Continue (Descend :: No) } else { ControlFlow :: Continue (Descend :: Yes) } }) ; let args = args . into_inner () . ok () ? ; let panic_expn = panic_expn . unwrap_or (PanicExpn :: Empty) ; Some ((args , panic_expn)) }
};
}
