// Generated macro for encountered_error (function)
macro_rules! Depcrate___macros_msg_send_null_errorencountered_error {
() => {
// Module: crate::__macros::msg_send::null_error
// Provides: {"encountered_error"}
// Dependencies: {}
# [cold] pub (crate) unsafe fn encountered_error < E : ClassType > (err : * mut E) -> Retained < E > { unsafe { Retained :: retain (err) } . unwrap_or_else (| | { let err = null_error () ; assert ! (E :: IS_NSERROR_COMPATIBLE) ; unsafe { Retained :: cast_unchecked (err) } }) }
};
}
