// Generated macro for impl_78 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_78 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; # [inline] fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . remaining == 0 { None } else { self . remaining -= 1 ; unsafe { let head = self . head . as_ptr () ; let (key , value) = (* head) . entry_mut () ; self . head = Some ((* head) . links . value . next) ; Some ((key , value)) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
