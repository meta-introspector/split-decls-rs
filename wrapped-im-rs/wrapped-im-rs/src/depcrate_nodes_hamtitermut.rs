// Generated macro for IterMut (struct)
macro_rules! Depcrate_nodes_hamtIterMut {
() => {
// Module: crate::nodes::hamt
// Provides: {"IterMut"}
// Dependencies: {}
pub (crate) struct IterMut < 'a , A > { count : usize , pool : Pool < Node < A > > , stack : Vec < ChunkIterMut < 'a , Entry < A > , HashWidth > > , current : ChunkIterMut < 'a , Entry < A > , HashWidth > , collision : Option < (HashBits , SliceIterMut < 'a , A >) > , }
};
}
