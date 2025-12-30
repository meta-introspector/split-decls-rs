// Generated macro for allocate_node (function)
macro_rules! Depcrate_linked_hash_mapallocate_node {
() => {
// Module: crate::linked_hash_map
// Provides: {"allocate_node"}
// Dependencies: {}
# [inline] unsafe fn allocate_node < K , V > (free_list : & mut Option < NonNull < Node < K , V > > >) -> NonNull < Node < K , V > > { if let Some (mut free) = pop_free (free_list) { free . as_mut () . links . value = ValueLinks { next : NonNull :: dangling () , prev : NonNull :: dangling () , } ; free } else { NonNull :: new_unchecked (Box :: into_raw (Box :: new (Node { entry : MaybeUninit :: uninit () , links : Links { value : ValueLinks { next : NonNull :: dangling () , prev : NonNull :: dangling () , } , } , }))) } }
};
}
