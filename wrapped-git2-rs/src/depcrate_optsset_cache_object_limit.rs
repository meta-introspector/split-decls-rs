// Generated macro for set_cache_object_limit (function)
macro_rules! Depcrate_optsset_cache_object_limit {
() => {
// Module: crate::opts
// Provides: {"set_cache_object_limit"}
// Dependencies: {}
# [doc = " Set the maximum data size for the given type of object to be considered"] # [doc = " eligible for caching in memory.  Setting to value to zero means that that"] # [doc = " type of object will not be cached.  Defaults to 0 for [`ObjectType::Blob`]"] # [doc = " (i.e. won't cache blobs) and 4k for [`ObjectType::Commit`],"] # [doc = " [`ObjectType::Tree`], and [`ObjectType::Tag`]."] # [doc = ""] # [doc = " `kind` must be one of [`ObjectType::Blob`], [`ObjectType::Commit`],"] # [doc = " [`ObjectType::Tree`], and [`ObjectType::Tag`]."] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is modifying a C global without synchronization, so it is not"] # [doc = " thread safe, and should only be called before any thread is spawned."] pub unsafe fn set_cache_object_limit (kind : ObjectType , size : libc :: size_t) -> Result < () , Error > { crate :: init () ; try_call ! (raw :: git_libgit2_opts (raw :: GIT_OPT_SET_CACHE_OBJECT_LIMIT as libc :: c_int , kind as libc :: c_int , size)) ; Ok (()) }
};
}
