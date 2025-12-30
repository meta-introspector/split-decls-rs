// Generated macro for Iter (struct)
macro_rules! Depcrate_nodes_hamtIter {
() => {
// Module: crate::nodes::hamt
// Provides: {"Iter"}
// Dependencies: {}
pub (crate) struct Iter < 'a , A > { count : usize , stack : Vec < ChunkIter < 'a , Entry < A > , HashWidth > > , current : ChunkIter < 'a , Entry < A > , HashWidth > , collision : Option < (HashBits , SliceIter < 'a , A >) > , }
};
}
