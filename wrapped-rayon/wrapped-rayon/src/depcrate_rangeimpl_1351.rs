// Generated macro for impl_1351 (impl)
macro_rules! Depcrate_rangeimpl_1351 {
() => {
// Module: crate::range
// Provides: {"impl_1351"}
// Dependencies: {}
impl < T > IntoIterator for IterProducer < T > where Range < T > : Iterator , { type Item = < Range < T > as Iterator > :: Item ; type IntoIter = Range < T > ; fn into_iter (self) -> Self :: IntoIter { self . range } }
};
}
