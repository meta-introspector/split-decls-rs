// Generated macro for impl_1309 (impl)
macro_rules! Depcrate_matrix_graphimpl_1309 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1309"}
// Dependencies: {}
impl < 'a , N : 'a , Ix : IndexType , S : BuildHasher > Iterator for NodeReferences < 'a , N , Ix , S > { type Item = (NodeIndex < Ix > , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| i | (NodeIndex :: new (i) , & self . nodes [i])) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
