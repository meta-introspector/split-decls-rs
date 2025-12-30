// Generated macro for impl_62 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_62 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_62"}
// Dependencies: {}
impl < K , V > IntoIter < K , V > { # [inline] pub (crate) fn iter (& self) -> Iter < '_ , K , V > { Iter { head : self . head . as_ptr () , tail : self . tail . as_ptr () , remaining : self . remaining , marker : PhantomData , } } }
};
}
