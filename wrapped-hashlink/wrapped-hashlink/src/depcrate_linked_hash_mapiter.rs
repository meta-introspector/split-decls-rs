// Generated macro for Iter (struct)
macro_rules! Depcrate_linked_hash_mapIter {
() => {
// Module: crate::linked_hash_map
// Provides: {"Iter"}
// Dependencies: {}
pub struct Iter < 'a , K , V > { head : * const Node < K , V > , tail : * const Node < K , V > , remaining : usize , marker : PhantomData < (& 'a K , & 'a V) > , }
};
}
