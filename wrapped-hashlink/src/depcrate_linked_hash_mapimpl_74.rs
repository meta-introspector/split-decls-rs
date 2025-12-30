// Generated macro for impl_74 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_74 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_74"}
// Dependencies: {}
impl < K , V > fmt :: Debug for IterMut < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
