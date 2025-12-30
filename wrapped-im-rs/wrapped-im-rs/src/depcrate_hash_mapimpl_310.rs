// Generated macro for impl_310 (impl)
macro_rules! Depcrate_hash_mapimpl_310 {
() => {
// Module: crate::hash::map
// Provides: {"impl_310"}
// Dependencies: {}
impl < K , V > HashValue for (K , V) where K : Eq , { type Key = K ; fn extract_key (& self) -> & Self :: Key { & self . 0 } fn ptr_eq (& self , _other : & Self) -> bool { false } }
};
}
