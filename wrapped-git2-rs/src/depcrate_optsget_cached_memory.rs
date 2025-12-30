// Generated macro for get_cached_memory (function)
macro_rules! Depcrate_optsget_cached_memory {
() => {
// Module: crate::opts
// Provides: {"get_cached_memory"}
// Dependencies: {}
# [doc = " Get the current bytes in cache and the maximum that would be allowed in the cache."] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is reading a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn get_cached_memory () -> Result < (libc :: ssize_t , libc :: ssize_t) , Error > { crate :: init () ; let mut current = 0 ; let mut allowed = 0 ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_GET_CACHED_MEMORY as libc :: c_int , & mut current , & mut allowed)) ; Ok ((current , allowed)) }
};
}
