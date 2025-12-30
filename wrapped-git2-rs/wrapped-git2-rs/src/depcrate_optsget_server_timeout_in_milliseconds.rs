// Generated macro for get_server_timeout_in_milliseconds (function)
macro_rules! Depcrate_optsget_server_timeout_in_milliseconds {
() => {
// Module: crate::opts
// Provides: {"get_server_timeout_in_milliseconds"}
// Dependencies: {}
# [doc = " Get server timeout in milliseconds"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn get_server_timeout_in_milliseconds () -> Result < libc :: c_int , Error > { crate :: init () ; let mut server_timeout = 0 ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_GET_SERVER_TIMEOUT as libc :: c_int , & mut server_timeout)) ; Ok (server_timeout) }
};
}
