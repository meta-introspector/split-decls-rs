// Generated macro for impl_443 (impl)
macro_rules! Depcrate_common_if_none_matchimpl_443 {
() => {
// Module: crate::common::if_none_match
// Provides: {"impl_443"}
// Dependencies: {}
impl IfNoneMatch { # [doc = " Create a new `If-None-Match: *` header."] pub fn any () -> IfNoneMatch { IfNoneMatch (EntityTagRange :: Any) } # [doc = " Checks whether the ETag passes this precondition."] pub fn precondition_passes (& self , etag : & ETag) -> bool { ! self . 0 . matches_weak (& etag . 0) } }
};
}
