// Generated macro for impl_76 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_76 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_76"}
// Dependencies: {}
impl < K , V > fmt :: Debug for Drain < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
