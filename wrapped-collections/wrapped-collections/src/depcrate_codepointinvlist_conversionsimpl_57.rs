// Generated macro for impl_57 (impl)
macro_rules! Depcrate_codepointinvlist_conversionsimpl_57 {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"impl_57"}
// Dependencies: {}
impl TryFrom < RangeFull > for CodePointInversionList < '_ > { type Error = RangeError ; fn try_from (_ : RangeFull) -> Result < Self , Self :: Error > { Ok (Self :: all ()) } }
};
}
