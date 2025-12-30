// Generated macro for set_up_statics (module)
macro_rules! Depcrateset_up_statics {
() => {
// Module: crate
// Provides: {"set_up_statics"}
// Dependencies: {}
# [cfg (all (feature = "override_allocator_on_supported_platforms" , not (target_vendor = "apple")))] mod set_up_statics { use super :: * ; # [used] static USED_MALLOC : unsafe extern "C" fn (usize) -> * mut c_void = malloc ; # [used] static USED_CALLOC : unsafe extern "C" fn (usize , usize) -> * mut c_void = calloc ; # [used] static USED_POSIX_MEMALIGN : unsafe extern "C" fn (* mut * mut c_void , usize , usize) -> c_int = posix_memalign ; # [used] static USED_ALIGNED_ALLOC : unsafe extern "C" fn (usize , usize) -> * mut c_void = aligned_alloc ; # [used] static USED_REALLOC : unsafe extern "C" fn (* mut c_void , usize) -> * mut c_void = realloc ; # [used] static USED_FREE : unsafe extern "C" fn (* mut c_void) = free ; }
};
}
