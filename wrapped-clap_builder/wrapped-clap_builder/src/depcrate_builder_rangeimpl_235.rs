// Generated macro for impl_235 (impl)
macro_rules! Depcrate_builder_rangeimpl_235 {
() => {
// Module: crate::builder::range
// Provides: {"impl_235"}
// Dependencies: {}
impl From < std :: ops :: Range < usize > > for ValueRange { fn from (range : std :: ops :: Range < usize >) -> Self { let start_inclusive = range . start ; let end_inclusive = range . end . saturating_sub (1) ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
