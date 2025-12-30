// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : Clone + PartialOrd + PartialEq > From < RangeInclusive < T > > for AnyRange < T > { fn from (r : RangeInclusive < T >) -> Self { Self :: RangeInclusive (r) } }
};
}
