// Generated macro for impl_122 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_122 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_122"}
// Dependencies: {}
impl < T > OptNonNullExt < T > for Option < NonNull < T > > { # [inline] fn as_ptr (self) -> * mut T { match self { Some (ptr) => ptr . as_ptr () , None => ptr :: null_mut () , } } }
};
}
