// Generated macro for impl_36 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_36 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_36"}
// Dependencies: {}
impl < K : fmt :: Debug , V , S > fmt :: Debug for VacantEntry < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
};
}
