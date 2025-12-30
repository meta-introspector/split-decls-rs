// Generated macro for check (function)
macro_rules! Depcrate_methods_chars_last_cmpcheck {
() => {
// Module: crate::methods::chars_last_cmp
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks for the `CHARS_LAST_CMP` lint."] pub (super) fn check (cx : & LateContext < '_ > , info : & crate :: methods :: BinaryExprInfo < '_ >) -> bool { if chars_cmp :: check (cx , info , & [sym :: chars , sym :: last] , CHARS_LAST_CMP , "ends_with") { true } else { chars_cmp :: check (cx , info , & [sym :: chars , sym :: next_back] , CHARS_LAST_CMP , "ends_with") } }
};
}
