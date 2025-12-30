// Generated macro for impl_1204 (impl)
macro_rules! Depcrate_graphmapimpl_1204 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1204"}
// Dependencies: {}
impl < 'a , N , E , Ty , S > Iterator for EdgesDirected < 'a , N , E , Ty , S > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , S : BuildHasher , { type Item = (N , N , & 'a E) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| mut b | { let mut a = self . from ; if self . dir == Direction :: Incoming { mem :: swap (& mut a , & mut b) ; } match self . edges . get (& GraphMap :: < N , E , Ty , S > :: edge_key (a , b)) { None => unreachable ! () , Some (edge) => (a , b , edge) , } }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
