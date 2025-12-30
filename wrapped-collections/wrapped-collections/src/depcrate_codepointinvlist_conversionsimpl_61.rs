// Generated macro for impl_61 (impl)
macro_rules! Depcrate_codepointinvlist_conversionsimpl_61 {
() => {
// Module: crate::codepointinvlist::conversions
// Provides: {"impl_61"}
// Dependencies: {}
impl FromIterator < RangeInclusive < u32 > > for CodePointInversionList < '_ > { fn from_iter < I : IntoIterator < Item = RangeInclusive < u32 > > > (iter : I) -> Self { let mut builder = CodePointInversionListBuilder :: new () ; for range in iter { builder . add_range32 (range) ; } builder . build () } }
};
}
