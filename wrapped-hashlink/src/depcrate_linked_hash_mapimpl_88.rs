// Generated macro for impl_88 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_88 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_88"}
// Dependencies: {}
impl < K , V > Drop for IntoIter < K , V > { # [inline] fn drop (& mut self) { for _ in 0 .. self . remaining { unsafe { let tail = self . tail . as_ptr () ; self . tail = Some ((* tail) . links . value . prev) ; (* tail) . take_entry () ; let _ = Box :: from_raw (tail) ; } } } }
};
}
