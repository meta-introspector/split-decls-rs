// Generated macro for check (function)
macro_rules! Depcrate_methods_chars_next_cmp_with_unwrapcheck {
() => {
// Module: crate::methods::chars_next_cmp_with_unwrap
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks for the `CHARS_NEXT_CMP` lint with `unwrap()`."] pub (super) fn check (cx : & LateContext < '_ > , info : & crate :: methods :: BinaryExprInfo < '_ >) -> bool { crate :: methods :: chars_cmp_with_unwrap :: check (cx , info , & [sym :: chars , sym :: next , sym :: unwrap] , CHARS_NEXT_CMP , "starts_with" ,) }
};
}
