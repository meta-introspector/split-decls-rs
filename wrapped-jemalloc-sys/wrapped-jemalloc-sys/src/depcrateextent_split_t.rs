// Generated macro for extent_split_t (type)
macro_rules! Depcrateextent_split_t {
() => {
// Module: crate
// Provides: {"extent_split_t"}
// Dependencies: {}
# [doc = " Extent split function."] # [doc = ""] # [doc = " Optionally splits an extent at given `addr` and `size` into two adjacent"] # [doc = " extents, the first of `size_a` bytes, and the second of `size_b` bytes,"] # [doc = " operating on `committed`/decommitted memory as indicated, on behalf of arena"] # [doc = " `arena_ind`, returning `false` upon success."] # [doc = ""] # [doc = " If the function returns `true`, this indicates that the extent remains"] # [doc = " unsplit and therefore should continue to be operated on as a whole."] pub type extent_split_t = unsafe extern "C" fn (extent_hooks : * mut extent_hooks_t , addr : * mut c_void , size : size_t , size_a : size_t , size_b : size_t , committed : c_bool , arena_ind : c_uint ,) -> c_bool ;
};
}
