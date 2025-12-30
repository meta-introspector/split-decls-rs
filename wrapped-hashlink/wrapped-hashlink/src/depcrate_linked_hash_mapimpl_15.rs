// Generated macro for impl_15 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_15 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_15"}
// Dependencies: {}
impl < K , V , S > fmt :: Debug for LinkedHashMap < K , V , S > where K : fmt :: Debug , V : fmt :: Debug , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_map () . entries (self) . finish () } }
};
}
