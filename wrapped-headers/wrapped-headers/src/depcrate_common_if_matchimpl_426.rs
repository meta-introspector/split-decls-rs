// Generated macro for impl_426 (impl)
macro_rules! Depcrate_common_if_matchimpl_426 {
() => {
// Module: crate::common::if_match
// Provides: {"impl_426"}
// Dependencies: {}
impl From < ETag > for IfMatch { fn from (etag : ETag) -> IfMatch { IfMatch (EntityTagRange :: Tags (HeaderValue :: from (etag . 0) . into ())) } }
};
}
