// Generated macro for impl_63 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_63 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_63"}
// Dependencies: {}
impl < K , V > Drain < '_ , K , V > { # [inline] pub (crate) fn iter (& self) -> Iter < '_ , K , V > { Iter { head : self . head . as_ptr () , tail : self . tail . as_ptr () , remaining : self . remaining , marker : PhantomData , } } }
};
}
