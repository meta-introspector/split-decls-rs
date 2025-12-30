// Generated macro for debug_assert_ne_ (function)
macro_rules! Depcratedebug_assert_ne_ {
() => {
// Module: crate
// Provides: {"debug_assert_ne_"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn debug_assert_ne_ (input : TokenStream) -> TokenStream { let assert = TokenStream2 :: from (assert_ne_ (input)) ; quote ! (if cfg ! (debug_assertions) { # assert }) . into () }
};
}
