// Generated macro for impl_237 (impl)
macro_rules! Depcrate_builder_rangeimpl_237 {
() => {
// Module: crate::builder::range
// Provides: {"impl_237"}
// Dependencies: {}
impl From < std :: ops :: RangeFrom < usize > > for ValueRange { fn from (range : std :: ops :: RangeFrom < usize >) -> Self { let start_inclusive = range . start ; let end_inclusive = usize :: MAX ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
