// Generated macro for impl_329 (impl)
macro_rules! Depcrate_iter_rangeimpl_329 {
() => {
// Module: crate::iter_range
// Provides: {"impl_329"}
// Dependencies: {}
impl < K : Into < Vec < u8 > > > IterateBounds for std :: ops :: RangeTo < K > { fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { (None , Some (self . end . into ())) } }
};
}
