// Generated macro for IterMut (struct)
macro_rules! Depcrate_linked_hash_mapIterMut {
() => {
// Module: crate::linked_hash_map
// Provides: {"IterMut"}
// Dependencies: {}
pub struct IterMut < 'a , K , V > { head : Option < NonNull < Node < K , V > > > , tail : Option < NonNull < Node < K , V > > > , remaining : usize , marker : PhantomData < (& 'a K , & 'a mut V) > , }
};
}
