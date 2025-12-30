// Generated macro for debug_assert_ (function)
macro_rules! Depcratedebug_assert_ {
() => {
// Module: crate
// Provides: {"debug_assert_"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn debug_assert_ (input : TokenStream) -> TokenStream { let assert = TokenStream2 :: from (assert_ (input)) ; quote ! (if cfg ! (debug_assertions) { # assert }) . into () }
};
}
