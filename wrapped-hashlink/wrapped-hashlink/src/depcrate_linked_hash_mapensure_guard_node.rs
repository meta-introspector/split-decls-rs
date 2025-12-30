// Generated macro for ensure_guard_node (function)
macro_rules! Depcrate_linked_hash_mapensure_guard_node {
() => {
// Module: crate::linked_hash_map
// Provides: {"ensure_guard_node"}
// Dependencies: {}
# [inline] unsafe fn ensure_guard_node < K , V > (head : & mut Option < NonNull < Node < K , V > > >) { if head . is_none () { let mut p = NonNull :: new_unchecked (Box :: into_raw (Box :: new (Node { entry : MaybeUninit :: uninit () , links : Links { value : ValueLinks { next : NonNull :: dangling () , prev : NonNull :: dangling () , } , } , }))) ; p . as_mut () . links . value = ValueLinks { next : p , prev : p } ; * head = Some (p) ; } }
};
}
