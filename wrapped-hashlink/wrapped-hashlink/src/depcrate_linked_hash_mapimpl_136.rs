// Generated macro for impl_136 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_136 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_136"}
// Dependencies: {}
impl < K , V > Drop for DropFilteredValues < '_ , K , V > { fn drop (& mut self) { unsafe { let end_free = self . cur_free ; while self . cur_free != * self . free { let cur_free = self . cur_free . as_ptr () ; (* cur_free) . take_entry () ; self . cur_free = (* cur_free) . links . free . next ; } * self . free = end_free ; } } }
};
}
