// Generated macro for impl_8 (impl)
macro_rules! Depcrate_cacheimpl_8 {
() => {
// Module: crate::cache
// Provides: {"impl_8"}
// Dependencies: {}
impl FileIdMap { # [doc = " Construct an empty cache."] pub fn new () -> Self { Default :: default () } fn dir_scan_depth (is_recursive : bool) -> usize { if is_recursive { usize :: MAX } else { 1 } } }
};
}
