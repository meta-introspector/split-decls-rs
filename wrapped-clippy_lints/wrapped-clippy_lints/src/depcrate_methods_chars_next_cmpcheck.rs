// Generated macro for check (function)
macro_rules! Depcrate_methods_chars_next_cmpcheck {
() => {
// Module: crate::methods::chars_next_cmp
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks for the `CHARS_NEXT_CMP` lint."] pub (super) fn check (cx : & LateContext < '_ > , info : & crate :: methods :: BinaryExprInfo < '_ >) -> bool { crate :: methods :: chars_cmp :: check (cx , info , & [sym :: chars , sym :: next] , CHARS_NEXT_CMP , "starts_with") }
};
}
