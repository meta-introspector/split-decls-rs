// Generated macro for is_eq (function)
macro_rules! Depcrate___macros_msg_send_null_erroris_eq {
() => {
// Module: crate::__macros::msg_send::null_error
// Provides: {"is_eq"}
// Dependencies: {}
# [doc = " Poor mans string equality in `const`. Implements `a == b`."] const fn is_eq (a : & str , b : & str) -> bool { let a = a . as_bytes () ; let b = b . as_bytes () ; if a . len () != b . len () { return false ; } let mut i = 0 ; while i < a . len () { if a [i] != b [i] { return false ; } i += 1 ; } true }
};
}
