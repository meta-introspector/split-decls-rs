// Generated macro for impl_93 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_93 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_93"}
// Dependencies: {}
impl < K : fmt :: Debug , V > fmt :: Debug for Keys < '_ , K , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
