// Generated macro for impl_444 (impl)
macro_rules! Depcrate_common_if_none_matchimpl_444 {
() => {
// Module: crate::common::if_none_match
// Provides: {"impl_444"}
// Dependencies: {}
impl From < ETag > for IfNoneMatch { fn from (etag : ETag) -> IfNoneMatch { IfNoneMatch (EntityTagRange :: Tags (HeaderValue :: from (etag . 0) . into ())) } }
};
}
