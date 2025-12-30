// Generated macro for drop_value_nodes (function)
macro_rules! Depcrate_linked_hash_mapdrop_value_nodes {
() => {
// Module: crate::linked_hash_map
// Provides: {"drop_value_nodes"}
// Dependencies: {}
# [inline] unsafe fn drop_value_nodes < K , V > (guard : NonNull < Node < K , V > >) { let mut cur = guard . as_ref () . links . value . prev ; while cur != guard { let prev = cur . as_ref () . links . value . prev ; cur . as_mut () . take_entry () ; let _ = Box :: from_raw (cur . as_ptr ()) ; cur = prev ; } }
};
}
