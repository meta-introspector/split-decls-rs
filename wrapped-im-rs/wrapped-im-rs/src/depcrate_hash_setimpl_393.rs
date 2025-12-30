// Generated macro for impl_393 (impl)
macro_rules! Depcrate_hash_setimpl_393 {
() => {
// Module: crate::hash::set
// Provides: {"impl_393"}
// Dependencies: {}
impl < A > HashValue for Value < A > where A : Hash + Eq , { type Key = A ; fn extract_key (& self) -> & Self :: Key { & self . 0 } fn ptr_eq (& self , _other : & Self) -> bool { false } }
};
}
