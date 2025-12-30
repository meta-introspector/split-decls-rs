// Generated macro for impl_105 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_105 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_105"}
// Dependencies: {}
impl < K , V > fmt :: Debug for ValuesMut < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter ()) . finish () } }
};
}
