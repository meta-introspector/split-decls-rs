// Generated macro for detach_node (function)
macro_rules! Depcrate_linked_hash_mapdetach_node {
() => {
// Module: crate::linked_hash_map
// Provides: {"detach_node"}
// Dependencies: {}
# [inline] unsafe fn detach_node < K , V > (mut node : NonNull < Node < K , V > >) { node . as_mut () . links . value . prev . as_mut () . links . value . next = node . as_ref () . links . value . next ; node . as_mut () . links . value . next . as_mut () . links . value . prev = node . as_ref () . links . value . prev ; }
};
}
