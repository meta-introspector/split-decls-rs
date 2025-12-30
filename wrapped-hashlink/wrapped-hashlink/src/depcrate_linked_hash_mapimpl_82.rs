// Generated macro for impl_82 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_82 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for IterMut < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . remaining == 0 { None } else { self . remaining -= 1 ; unsafe { let tail = self . tail . as_ptr () ; self . tail = Some ((* tail) . links . value . prev) ; let (key , value) = (* tail) . entry_mut () ; Some ((key , value)) } } } }
};
}
