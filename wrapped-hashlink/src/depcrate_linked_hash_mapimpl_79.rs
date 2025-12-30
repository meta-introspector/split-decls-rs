// Generated macro for impl_79 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_79 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_79"}
// Dependencies: {}
impl < K , V > Iterator for IntoIter < K , V > { type Item = (K , V) ; # [inline] fn next (& mut self) -> Option < (K , V) > { if self . remaining == 0 { return None ; } self . remaining -= 1 ; unsafe { let head = self . head . as_ptr () ; self . head = Some ((* head) . links . value . next) ; let mut e = Box :: from_raw (head) ; Some (e . take_entry ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
