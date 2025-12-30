// Generated macro for get_expr_int (function)
macro_rules! Depcrate_make_uleget_expr_int {
() => {
// Module: crate::make_ule
// Provides: {"get_expr_int"}
// Dependencies: {}
fn get_expr_int (e : & Expr) -> Option < u64 > { if let Ok (Lit :: Int (ref i)) = syn :: parse2 (quote ! (# e)) { return i . base10_parse () . ok () ; } None }
};
}
