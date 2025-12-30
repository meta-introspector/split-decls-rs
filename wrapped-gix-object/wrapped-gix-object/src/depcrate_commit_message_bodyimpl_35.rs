// Generated macro for impl_35 (impl)
macro_rules! Depcrate_commit_message_bodyimpl_35 {
() => {
// Module: crate::commit::message::body
// Provides: {"impl_35"}
// Dependencies: {}
# [doc = " Convenience methods"] impl < 'a > Trailers < 'a > { # [doc = " Filter trailers to only include `Signed-off-by` entries."] pub fn signed_off_by (self) -> impl Iterator < Item = TrailerRef < 'a > > { self . filter (TrailerRef :: is_signed_off_by) } # [doc = " Filter trailers to only include `Co-authored-by` entries."] pub fn co_authored_by (self) -> impl Iterator < Item = TrailerRef < 'a > > { self . filter (TrailerRef :: is_co_authored_by) } # [doc = " Filter trailers to only include attribution-related entries."] # [doc = " (`Signed-off-by`, `Co-authored-by`, `Acked-by`, `Reviewed-by`, `Tested-by`)."] pub fn attributions (self) -> impl Iterator < Item = TrailerRef < 'a > > { self . filter (TrailerRef :: is_attribution) } # [doc = " Filter trailers to only include authors from `Signed-off-by` and `Co-authored-by` entries."] pub fn authors (self) -> impl Iterator < Item = TrailerRef < 'a > > { self . filter (| trailer | trailer . is_signed_off_by () || trailer . is_co_authored_by ()) } }
};
}
