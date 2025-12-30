// Generated macro for impl_56 (impl)
macro_rules! Depcrate_codepointinvlist_conversionsimpl_56 {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"impl_56"}
// Dependencies: {}
impl TryFrom < RangeFrom < char > > for CodePointInversionList < '_ > { type Error = RangeError ; fn try_from (range : RangeFrom < char >) -> Result < Self , Self :: Error > { try_from_range (range) } }
};
}
