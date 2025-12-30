// Generated macro for IntoIter (struct)
macro_rules! Depcrate_linked_hash_mapIntoIter {
() => {
// Module: crate::linked_hash_map
// Provides: {"IntoIter"}
// Dependencies: {}
pub struct IntoIter < K , V > { head : Option < NonNull < Node < K , V > > > , tail : Option < NonNull < Node < K , V > > > , remaining : usize , marker : PhantomData < (K , V) > , }
};
}
