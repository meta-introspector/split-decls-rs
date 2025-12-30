// Generated macro for impl_238 (impl)
macro_rules! Depcrate_builder_rangeimpl_238 {
() => {
// Module: crate::builder::range
// Provides: {"impl_238"}
// Dependencies: {}
impl From < std :: ops :: RangeTo < usize > > for ValueRange { fn from (range : std :: ops :: RangeTo < usize >) -> Self { let start_inclusive = 0 ; let end_inclusive = range . end . saturating_sub (1) ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
