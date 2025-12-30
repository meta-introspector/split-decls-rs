// Generated macro for impl_3027 (impl)
macro_rules! Depcrate_proptestimpl_3027 {
() => {
// Module: crate::proptest
// Provides: {"impl_3027"}
// Dependencies: {}
impl < D : Dim > DimRange < D > { # [doc = " Converts the `DimRange` into an instance of `RangeInclusive`."] pub fn to_range_inclusive (& self) -> RangeInclusive < usize > { self . lower_bound () . value () ..= self . upper_bound () . value () } }
};
}
