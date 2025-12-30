// Generated macro for impl_310 (impl)
macro_rules! Depcrate___macros_msg_send_null_errorimpl_310 {
() => {
// Module: crate::__macros::msg_send::null_error
// Provides: {"impl_310"}
// Dependencies: {}
impl < T : ClassType > IsNSError for T { const IS_NSERROR_COMPATIBLE : bool = { if is_eq (T :: NAME , "NSError") || is_eq (T :: NAME , "NSObject") { true } else { panic ! ("error parameter must be either `NSError` or `NSObject`") } } ; }
};
}
