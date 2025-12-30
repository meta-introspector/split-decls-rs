// Generated macro for extent_merge_t (type)
macro_rules! Depcrateextent_merge_t {
() => {
// Module: crate
// Provides: {"extent_merge_t"}
// Dependencies: {}
# [doc = " Extent merge function."] # [doc = ""] # [doc = " Optionally merges adjacent extents, at given `addr_a` and `size_a` with given"] # [doc = " `addr_b` and `size_b` into one contiguous extent, operating on"] # [doc = " `committed`/decommitted memory as indicated, on behalf of arena `arena_ind`,"] # [doc = " returning `false` upon success."] # [doc = ""] # [doc = " If the function returns `true`, this indicates that the extents remain"] # [doc = " distinct mappings and therefore should continue to be operated on"] # [doc = " independently."] pub type extent_merge_t = unsafe extern "C" fn (extent_hooks : * mut extent_hooks_t , addr_a : * mut c_void , size_a : size_t , addr_b : * mut c_void , size_b : size_t , committed : c_bool , arena_ind : c_uint ,) -> c_bool ;
};
}
