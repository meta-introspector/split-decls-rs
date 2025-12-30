// Generated macro for impl_3026 (impl)
macro_rules! Depcrate_proptestimpl_3026 {
() => {
// Module: crate::proptest
// Provides: {"impl_3026"}
// Dependencies: {}
impl From < RangeInclusive < usize > > for DimRange < Dyn > { fn from (range : RangeInclusive < usize >) -> Self { DimRange :: from (Dyn (* range . start ()) ..= Dyn (* range . end ())) } }
};
}
