// Generated macro for impl_240 (impl)
macro_rules! Depcrate_builder_rangeimpl_240 {
() => {
// Module: crate::builder::range
// Provides: {"impl_240"}
// Dependencies: {}
impl From < std :: ops :: RangeToInclusive < usize > > for ValueRange { fn from (range : std :: ops :: RangeToInclusive < usize >) -> Self { let start_inclusive = 0 ; let end_inclusive = range . end ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
