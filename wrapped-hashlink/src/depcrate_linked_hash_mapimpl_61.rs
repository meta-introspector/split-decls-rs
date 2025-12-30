// Generated macro for impl_61 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_61 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_61"}
// Dependencies: {}
impl < K , V > IterMut < '_ , K , V > { # [inline] pub (crate) fn iter (& self) -> Iter < '_ , K , V > { Iter { head : self . head . as_ptr () , tail : self . tail . as_ptr () , remaining : self . remaining , marker : PhantomData , } } }
};
}
