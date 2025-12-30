// Generated macro for pop_free (function)
macro_rules! Depcrate_linked_hash_mappop_free {
() => {
// Module: crate::linked_hash_map
// Provides: {"pop_free"}
// Dependencies: {}
# [inline] unsafe fn pop_free < K , V > (free_list : & mut Option < NonNull < Node < K , V > > > ,) -> Option < NonNull < Node < K , V > > > { if let Some (free) = * free_list { * free_list = free . as_ref () . links . free . next ; Some (free) } else { None } }
};
}
