// Generated macro for impl_83 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_83 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_83"}
// Dependencies: {}
impl < K , V > DoubleEndedIterator for IntoIter < K , V > { # [inline] fn next_back (& mut self) -> Option < (K , V) > { if self . remaining == 0 { return None ; } self . remaining -= 1 ; unsafe { let mut e = * Box :: from_raw (self . tail . as_ptr ()) ; self . tail = Some (e . links . value . prev) ; Some (e . take_entry ()) } } }
};
}
