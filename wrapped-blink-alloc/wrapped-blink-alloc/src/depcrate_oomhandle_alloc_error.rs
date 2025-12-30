// Generated macro for handle_alloc_error (function)
macro_rules! Depcrate_oomhandle_alloc_error {
() => {
// Module: crate::oom
// Provides: {"handle_alloc_error"}
// Dependencies: {}
# [cfg_attr (feature = "alloc" , inline (always))] # [cfg_attr (not (feature = "alloc") , inline (never))] # [cold] pub (crate) fn handle_alloc_error (layout : core :: alloc :: Layout) -> ! { # [cfg (feature = "alloc")] alloc :: alloc :: handle_alloc_error (layout) ; # [cfg (not (feature = "alloc"))] panic ! ("allocation of {:?} failed" , layout) ; }
};
}
