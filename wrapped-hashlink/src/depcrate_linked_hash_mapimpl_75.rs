// Generated macro for impl_75 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_75 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_75"}
// Dependencies: {}
impl < K , V > fmt :: Debug for IntoIter < K , V > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
