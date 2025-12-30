// Generated macro for impl_135 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_135 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_135"}
// Dependencies: {}
impl < K , V > DropFilteredValues < '_ , K , V > { # [inline] fn drop_later (& mut self , node : NonNull < Node < K , V > >) { unsafe { detach_node (node) ; push_free (& mut self . cur_free , node) ; } } }
};
}
