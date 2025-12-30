// Generated macro for attach_before (function)
macro_rules! Depcrate_linked_hash_mapattach_before {
() => {
// Module: crate::linked_hash_map
// Provides: {"attach_before"}
// Dependencies: {}
# [inline] unsafe fn attach_before < K , V > (mut to_attach : NonNull < Node < K , V > > , mut node : NonNull < Node < K , V > >) { to_attach . as_mut () . links . value = ValueLinks { prev : node . as_ref () . links . value . prev , next : node , } ; node . as_mut () . links . value . prev = to_attach ; (* to_attach . as_mut () . links . value . prev . as_ptr ()) . links . value . next = to_attach ; }
};
}
