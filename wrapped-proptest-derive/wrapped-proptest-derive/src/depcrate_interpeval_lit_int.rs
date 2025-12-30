// Generated macro for eval_lit_int (function)
macro_rules! Depcrate_interpeval_lit_int {
() => {
// Module: crate::interp
// Provides: {"eval_lit_int"}
// Dependencies: {}
# [doc = " Interprets an integer literal."] fn eval_lit_int (lit : & syn :: LitInt) -> Option < u128 > { let lit = lit . to_string () ; eval_str_int (& lit) }
};
}
