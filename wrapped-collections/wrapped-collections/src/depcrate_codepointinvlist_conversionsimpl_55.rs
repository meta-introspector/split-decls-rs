// Generated macro for impl_55 (impl)
macro_rules! Depcrate_codepointinvlist_conversionsimpl_55 {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"impl_55"}
// Dependencies: {}
impl TryFrom < Range < char > > for CodePointInversionList < '_ > { type Error = RangeError ; fn try_from (range : Range < char >) -> Result < Self , Self :: Error > { try_from_range (range) } }
};
}
