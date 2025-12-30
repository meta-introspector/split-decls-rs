// Generated macro for debug_assert_eq_ (function)
macro_rules! Depcratedebug_assert_eq_ {
() => {
// Module: crate
// Provides: {"debug_assert_eq_"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn debug_assert_eq_ (input : TokenStream) -> TokenStream { let assert = TokenStream2 :: from (assert_eq_ (input)) ; quote ! (if cfg ! (debug_assertions) { # assert }) . into () }
};
}
