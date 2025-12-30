// Generated macro for impl_327 (impl)
macro_rules! Depcrate_iter_rangeimpl_327 {
() => {
// Module: crate::iter_range
// Provides: {"impl_327"}
// Dependencies: {}
impl < K : Into < Vec < u8 > > > IterateBounds for std :: ops :: Range < K > { fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { (Some (self . start . into ()) , Some (self . end . into ())) } }
};
}
