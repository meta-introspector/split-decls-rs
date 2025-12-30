// Generated macro for impl_328 (impl)
macro_rules! Depcrate_iter_rangeimpl_328 {
() => {
// Module: crate::iter_range
// Provides: {"impl_328"}
// Dependencies: {}
impl < K : Into < Vec < u8 > > > IterateBounds for std :: ops :: RangeFrom < K > { fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { (Some (self . start . into ()) , None) } }
};
}
