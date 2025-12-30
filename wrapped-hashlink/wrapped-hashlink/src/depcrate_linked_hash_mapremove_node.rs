// Generated macro for remove_node (function)
macro_rules! Depcrate_linked_hash_mapremove_node {
() => {
// Module: crate::linked_hash_map
// Provides: {"remove_node"}
// Dependencies: {}
# [inline] unsafe fn remove_node < K , V > (free_list : & mut Option < NonNull < Node < K , V > > > , mut node : NonNull < Node < K , V > > ,) -> (K , V) { detach_node (node) ; push_free (free_list , node) ; node . as_mut () . take_entry () }
};
}
