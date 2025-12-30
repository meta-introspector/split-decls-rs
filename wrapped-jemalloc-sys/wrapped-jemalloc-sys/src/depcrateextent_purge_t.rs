// Generated macro for extent_purge_t (type)
macro_rules! Depcrateextent_purge_t {
() => {
// Module: crate
// Provides: {"extent_purge_t"}
// Dependencies: {}
# [doc = " Extent purge function."] # [doc = ""] # [doc = " Discards physical pages within the virtual memory mapping associated with an"] # [doc = " extent at given `addr` and `size` at `offset` bytes, extending for `length` on"] # [doc = " behalf of arena `arena_ind`."] # [doc = ""] # [doc = " A lazy extent purge function (e.g. implemented via `madvise(...MADV_FREE)`)"] # [doc = " can delay purging indefinitely and leave the pages within the purged virtual"] # [doc = " memory range in an indeterminite state, whereas a forced extent purge"] # [doc = " function immediately purges, and the pages within the virtual memory range"] # [doc = " will be zero-filled the next time they are accessed. If the function returns"] # [doc = " `true`, this indicates failure to purge."] pub type extent_purge_t = unsafe extern "C" fn (extent_hooks : * mut extent_hooks_t , addr : * mut c_void , size : size_t , offset : size_t , length : size_t , arena_ind : c_uint ,) -> c_bool ;
};
}
