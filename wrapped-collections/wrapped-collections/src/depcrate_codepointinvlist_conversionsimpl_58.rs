// Generated macro for impl_58 (impl)
macro_rules! Depcrate_codepointinvlist_conversionsimpl_58 {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"impl_58"}
// Dependencies: {}
impl TryFrom < RangeInclusive < char > > for CodePointInversionList < '_ > { type Error = RangeError ; fn try_from (range : RangeInclusive < char >) -> Result < Self , Self :: Error > { try_from_range (range) } }
};
}
