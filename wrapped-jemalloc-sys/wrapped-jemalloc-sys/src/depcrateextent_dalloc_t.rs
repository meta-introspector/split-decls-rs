// Generated macro for extent_dalloc_t (type)
macro_rules! Depcrateextent_dalloc_t {
() => {
// Module: crate
// Provides: {"extent_dalloc_t"}
// Dependencies: {}
# [doc = " Extent deallocation function."] # [doc = ""] # [doc = " Deallocates an extent at given `addr` and `size` with `committed`/decommited"] # [doc = " memory as indicated, on behalf of arena `arena_ind`, returning `false` upon"] # [doc = " success."] # [doc = ""] # [doc = " If the function returns `true`, this indicates opt-out from deallocation;"] # [doc = " the virtual memory mapping associated with the extent remains mapped, in the"] # [doc = " same commit state, and available for future use, in which case it will be"] # [doc = " automatically retained for later reuse."] pub type extent_dalloc_t = unsafe extern "C" fn (extent_hooks : * mut extent_hooks_t , addr : * mut c_void , size : size_t , committed : c_bool , arena_ind : c_uint ,) -> c_bool ;
};
}
