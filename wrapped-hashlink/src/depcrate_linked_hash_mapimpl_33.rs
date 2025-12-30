// Generated macro for impl_33 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_33 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_33"}
// Dependencies: {}
impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for OccupiedEntry < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
};
}
