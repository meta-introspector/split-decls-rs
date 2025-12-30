// Generated macro for log_callback_get (function)
macro_rules! Depcrate_userdatalog_callback_get {
() => {
// Module: crate::userdata
// Provides: {"log_callback_get"}
// Dependencies: {}
# [cfg (not (feature = "no_log_capture"))] pub (crate) fn log_callback_get () -> Result < (rustls_log_callback , * mut c_void) , UserdataError > { USERDATA . try_with (| userdata | { userdata . try_borrow_mut () . map_or_else (| _ | Err (UserdataError :: AlreadyBorrowed) , | v | match v . last () { Some (u) => Ok ((u . log_callback , u . userdata)) , None => Err (UserdataError :: EmptyStack) , } ,) }) . unwrap_or (Err (UserdataError :: AccessError)) }
};
}
