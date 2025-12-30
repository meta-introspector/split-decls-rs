// Generated macro for impl_60 (impl)
macro_rules! Depcrate_codepointinvlist_conversionsimpl_60 {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"impl_60"}
// Dependencies: {}
impl TryFrom < RangeToInclusive < char > > for CodePointInversionList < '_ > { type Error = RangeError ; fn try_from (range : RangeToInclusive < char >) -> Result < Self , Self :: Error > { try_from_range (range) } }
};
}
