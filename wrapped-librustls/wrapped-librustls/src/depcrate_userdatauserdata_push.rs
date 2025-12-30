// Generated macro for userdata_push (function)
macro_rules! Depcrate_userdatauserdata_push {
() => {
// Module: crate::userdata
// Provides: {"userdata_push"}
// Dependencies: {}
# [must_use = "If you drop the guard, userdata will be immediately cleared"] pub (crate) fn userdata_push (u : * mut c_void , _cb : rustls_log_callback ,) -> Result < UserdataGuard , UserdataError > { USERDATA . try_with (| userdata | { userdata . try_borrow_mut () . map_or_else (| _ | Err (UserdataError :: AlreadyBorrowed) , | mut v | { v . push (Userdata { userdata : u , # [cfg (not (feature = "no_log_capture"))] log_callback : _cb , }) ; Ok (()) } ,) }) . unwrap_or (Err (UserdataError :: AccessError)) ? ; Ok (UserdataGuard :: new (u)) }
};
}
