// Generated macro for impl_425 (impl)
macro_rules! Depcrate_common_if_matchimpl_425 {
() => {
// Module: crate::common::if_match
// Provides: {"impl_425"}
// Dependencies: {}
impl IfMatch { # [doc = " Create a new `If-Match: *` header."] pub fn any () -> IfMatch { IfMatch (EntityTagRange :: Any) } # [doc = " Returns whether this is `If-Match: *`, matching any entity tag."] pub fn is_any (& self) -> bool { match self . 0 { EntityTagRange :: Any => true , EntityTagRange :: Tags (..) => false , } } # [doc = " Checks whether the `ETag` strongly matches."] pub fn precondition_passes (& self , etag : & ETag) -> bool { self . 0 . matches_strong (& etag . 0) } }
};
}
