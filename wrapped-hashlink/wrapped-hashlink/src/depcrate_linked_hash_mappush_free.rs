// Generated macro for push_free (function)
macro_rules! Depcrate_linked_hash_mappush_free {
() => {
// Module: crate::linked_hash_map
// Provides: {"push_free"}
// Dependencies: {}
# [inline] unsafe fn push_free < K , V > (free_list : & mut Option < NonNull < Node < K , V > > > , mut node : NonNull < Node < K , V > > ,) { node . as_mut () . links . free . next = * free_list ; * free_list = Some (node) ; }
};
}
