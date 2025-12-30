// Generated macro for eval_lit_verbatim (function)
macro_rules! Depcrate_interpeval_lit_verbatim {
() => {
// Module: crate::interp
// Provides: {"eval_lit_verbatim"}
// Dependencies: {}
# [doc = " Interprets a verbatim literal."] fn eval_lit_verbatim (lit : & proc_macro2 :: Literal) -> Option < u128 > { let lit = lit . to_string () ; eval_str_int (& lit) }
};
}
