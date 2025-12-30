// Generated macro for impl_59 (impl)
macro_rules! Depcrate_codepointinvlist_conversionsimpl_59 {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"impl_59"}
// Dependencies: {}
impl TryFrom < RangeTo < char > > for CodePointInversionList < '_ > { type Error = RangeError ; fn try_from (range : RangeTo < char >) -> Result < Self , Self :: Error > { try_from_range (range) } }
};
}
