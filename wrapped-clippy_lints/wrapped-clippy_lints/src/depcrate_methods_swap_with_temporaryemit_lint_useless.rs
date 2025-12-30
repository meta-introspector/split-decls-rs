// Generated macro for emit_lint_useless (function)
macro_rules! Depcrate_methods_swap_with_temporaryemit_lint_useless {
() => {
// Module: crate::methods::swap_with_temporary
// Provides: {"emit_lint_useless"}
// Dependencies: {}
fn emit_lint_useless (cx : & LateContext < '_ > , expr : & Expr < '_ > , left : & Expr < '_ > , right : & Expr < '_ > , left_temp : & Expr < '_ > , right_temp : & Expr < '_ > ,) { span_lint_and_then (cx , SWAP_WITH_TEMPORARY , expr . span , "swapping temporary values has no effect" , | diag | { emit_note (diag , expr , left , left_temp) ; emit_note (diag , expr , right , right_temp) ; } ,) ; }
};
}
