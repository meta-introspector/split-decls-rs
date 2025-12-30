// Generated macro for extent_decommit_t (type)
macro_rules! Depcrateextent_decommit_t {
() => {
// Module: crate
// Provides: {"extent_decommit_t"}
// Dependencies: {}
# [doc = " Extent decommit function."] # [doc = ""] # [doc = " Decommits any physical memory that is backing pages within an extent at"] # [doc = " given `addr` and `size` at `offset` bytes, extending for `length` on behalf of arena"] # [doc = " `arena_ind`, returning `false` upon success, in which case the pages will be"] # [doc = " committed via the extent commit function before being reused."] # [doc = ""] # [doc = " If the function returns `true`, this indicates opt-out from decommit; the"] # [doc = " memory remains committed and available for future use, in which case it will"] # [doc = " be automatically retained for later reuse."] pub type extent_decommit_t = unsafe extern "C" fn (extent_hooks : * mut extent_hooks_t , addr : * mut c_void , size : size_t , offset : size_t , length : size_t , arena_ind : c_uint ,) -> c_bool ;
};
}
