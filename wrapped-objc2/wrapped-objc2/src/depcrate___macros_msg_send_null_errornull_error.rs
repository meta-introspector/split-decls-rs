// Generated macro for null_error (function)
macro_rules! Depcrate___macros_msg_send_null_errornull_error {
() => {
// Module: crate::__macros::msg_send::null_error
// Provides: {"null_error"}
// Dependencies: {}
# [cold] fn null_error () -> Retained < NSObject > { static CACHED_NULL_ERROR : OnceLock < NSErrorWrapper > = OnceLock :: new () ; CACHED_NULL_ERROR . get_or_init (create_null_error) . 0 . clone () }
};
}
