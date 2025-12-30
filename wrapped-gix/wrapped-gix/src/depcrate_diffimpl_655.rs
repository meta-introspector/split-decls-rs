// Generated macro for impl_655 (impl)
macro_rules! Depcrate_diffimpl_655 {
() => {
// Module: crate::diff
// Provides: {"impl_655"}
// Dependencies: {}
# [cfg (feature = "blob-diff")] impl From < Options > for gix_diff :: tree_with_rewrites :: Options { fn from (opts : Options) -> Self { gix_diff :: tree_with_rewrites :: Options { location : opts . location , # [cfg (feature = "blob-diff")] rewrites : opts . rewrites , } } }
};
}
