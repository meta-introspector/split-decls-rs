// Generated macro for extent_commit_t (type)
macro_rules! Depcrateextent_commit_t {
() => {
// Module: crate
// Provides: {"extent_commit_t"}
// Dependencies: {}
# [doc = " Extent commit function."] # [doc = ""] # [doc = " Commits zeroed physical memory to back pages within an extent at given"] # [doc = " `addr` and `size` at `offset` bytes, extending for `length` on behalf of"] # [doc = " arena `arena_ind`, returning `false` upon success."] # [doc = ""] # [doc = " Committed memory may be committed in absolute terms as on a system that does"] # [doc = " not overcommit, or in implicit terms as on a system that overcommits and"] # [doc = " satisfies physical memory needs on demand via soft page faults. If the"] # [doc = " function returns `true`, this indicates insufficient physical memory to"] # [doc = " satisfy the request."] pub type extent_commit_t = unsafe extern "C" fn (extent_hooks : * mut extent_hooks_t , addr : * mut c_void , size : size_t , offset : size_t , length : size_t , arena_ind : c_uint ,) -> c_bool ;
};
}
