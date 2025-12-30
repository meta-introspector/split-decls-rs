// Generated macro for get_mwindow_size (function)
macro_rules! Depcrate_optsget_mwindow_size {
() => {
// Module: crate::opts
// Provides: {"get_mwindow_size"}
// Dependencies: {}
# [doc = " Get the maximum mmap window size"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is reading a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn get_mwindow_size () -> Result < libc :: size_t , Error > { crate :: init () ; let mut size = 0 ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_GET_MWINDOW_SIZE as libc :: c_int , & mut size)) ; Ok (size) }
};
}
