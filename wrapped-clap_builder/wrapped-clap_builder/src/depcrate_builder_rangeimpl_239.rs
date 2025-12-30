// Generated macro for impl_239 (impl)
macro_rules! Depcrate_builder_rangeimpl_239 {
() => {
// Module: crate::builder::range
// Provides: {"impl_239"}
// Dependencies: {}
impl From < std :: ops :: RangeInclusive < usize > > for ValueRange { fn from (range : std :: ops :: RangeInclusive < usize >) -> Self { let start_inclusive = * range . start () ; let end_inclusive = * range . end () ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
