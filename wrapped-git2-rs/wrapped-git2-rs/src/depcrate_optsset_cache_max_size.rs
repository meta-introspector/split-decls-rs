// Generated macro for set_cache_max_size (function)
macro_rules! Depcrate_optsset_cache_max_size {
() => {
// Module: crate::opts
// Provides: {"set_cache_max_size"}
// Dependencies: {}
# [doc = " Set the maximum total data size that will be cached in memory across all"] # [doc = " repositories before libgit2 starts evicting objects from the cache. This"] # [doc = " is a soft limit, in that the library might briefly exceed it, but will start"] # [doc = " aggressively evicting objects from cache when that happens. The default"] # [doc = " cache size is 256MB."] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn set_cache_max_size (size : libc :: ssize_t) -> Result < () , Error > { crate :: init () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_CACHE_MAX_SIZE as libc :: c_int , size)) ; Ok (()) }
};
}
