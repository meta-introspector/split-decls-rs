// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : Clone + PartialOrd + PartialEq > From < RangeToInclusive < T > > for AnyRange < T > { fn from (r : RangeToInclusive < T >) -> Self { Self :: RangeToInclusive (r) } }
};
}
