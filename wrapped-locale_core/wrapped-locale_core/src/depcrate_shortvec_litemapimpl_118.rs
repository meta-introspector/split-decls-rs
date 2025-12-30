// Generated macro for impl_118 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_118 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_118"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K : Ord , V > StoreFromIterable < K , V > for ShortBoxSlice < (K , V) > { fn lm_sort_from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { alloc :: vec :: Vec :: lm_sort_from_iter (iter) . into () } }
};
}
