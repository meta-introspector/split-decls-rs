// Generated macro for impl_73 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_73 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_73"}
// Dependencies: {}
impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for Iter < '_ , K , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
