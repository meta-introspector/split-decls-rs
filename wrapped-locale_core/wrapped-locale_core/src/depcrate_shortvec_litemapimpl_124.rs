// Generated macro for impl_124 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_124 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K , V > StoreIntoIterator < K , V > for ShortBoxSlice < (K , V) > { type KeyValueIntoIter = ShortBoxSliceIntoIter < (K , V) > ; fn lm_into_iter (self) -> Self :: KeyValueIntoIter { self . into_iter () } }
};
}
