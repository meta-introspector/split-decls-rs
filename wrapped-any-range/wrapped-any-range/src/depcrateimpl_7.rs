// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < T : Clone + PartialOrd + PartialEq > RangeBounds < T > for AnyRange < T > { fn start_bound (& self) -> Bound < & T > { match self { Self :: Range (r) => r . start_bound () , Self :: RangeFrom (r) => r . start_bound () , Self :: RangeFull (r) => r . start_bound () , Self :: RangeInclusive (r) => r . start_bound () , Self :: RangeTo (r) => r . start_bound () , Self :: RangeToInclusive (r) => r . start_bound () , } } fn end_bound (& self) -> Bound < & T > { match self { Self :: Range (r) => r . end_bound () , Self :: RangeFrom (r) => r . end_bound () , Self :: RangeFull (r) => r . end_bound () , Self :: RangeInclusive (r) => r . end_bound () , Self :: RangeTo (r) => r . end_bound () , Self :: RangeToInclusive (r) => r . end_bound () , } } }
};
}
