// Generated macro for impl_279 (impl)
macro_rules! Depcrate_ord_setimpl_279 {
() => {
// Module: crate::ord::set
// Provides: {"impl_279"}
// Dependencies: {}
impl < A > IntoIterator for OrdSet < A > where A : Ord + Clone , { type Item = A ; type IntoIter = ConsumingIter < A > ; fn into_iter (self) -> Self :: IntoIter { ConsumingIter { it : ConsumingNodeIter :: new (& self . root , self . size) , } } }
};
}
