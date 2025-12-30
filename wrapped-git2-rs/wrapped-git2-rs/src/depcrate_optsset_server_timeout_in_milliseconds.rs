// Generated macro for set_server_timeout_in_milliseconds (function)
macro_rules! Depcrate_optsset_server_timeout_in_milliseconds {
() => {
// Module: crate::opts
// Provides: {"set_server_timeout_in_milliseconds"}
// Dependencies: {}
# [doc = " Set server timeout in milliseconds"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn set_server_timeout_in_milliseconds (timeout : libc :: c_int) -> Result < () , Error > { crate :: init () ; let error = raw :: git_libgit2_opts (raw :: GIT_OPT_SET_SERVER_TIMEOUT as libc :: c_int , timeout as libc :: c_int ,) ; debug_assert ! (error >= 0) ; Ok (()) }
};
}
