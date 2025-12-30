// Generated macro for extent_destroy_t (type)
macro_rules! Depcrateextent_destroy_t {
() => {
// Module: crate
// Provides: {"extent_destroy_t"}
// Dependencies: {}
# [doc = " Extent destruction function."] # [doc = ""] # [doc = " Unconditionally destroys an extent at given `addr` and `size` with"] # [doc = " `committed`/decommited memory as indicated, on behalf of arena `arena_ind`."] # [doc = ""] # [doc = " This function may be called to destroy retained extents during arena"] # [doc = " destruction (see `arena.<i>.destroy`)."] pub type extent_destroy_t = unsafe extern "C" fn (extent_hooks : * mut extent_hooks_t , addr : * mut c_void , size : size_t , committed : c_bool , arena_ind : c_uint ,) ;
};
}
