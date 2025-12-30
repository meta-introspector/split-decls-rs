// Generated macro for impl_100 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_100 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_100"}
// Dependencies: {}
impl < K , V : fmt :: Debug > fmt :: Debug for Values < '_ , K , V > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
