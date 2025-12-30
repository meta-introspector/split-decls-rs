// Generated macro for drop_free_nodes (function)
macro_rules! Depcrate_linked_hash_mapdrop_free_nodes {
() => {
// Module: crate::linked_hash_map
// Provides: {"drop_free_nodes"}
// Dependencies: {}
# [inline] unsafe fn drop_free_nodes < K , V > (mut free : Option < NonNull < Node < K , V > > >) { while let Some (some_free) = free { let next_free = some_free . as_ref () . links . free . next ; let _ = Box :: from_raw (some_free . as_ptr ()) ; free = next_free ; } }
};
}
