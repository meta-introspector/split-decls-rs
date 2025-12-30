// Generated macro for impl_1207 (impl)
macro_rules! Depcrate_graphmapimpl_1207 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1207"}
// Dependencies: {}
impl < 'a , N , E , Ty > DoubleEndedIterator for AllEdges < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (& (n1 , n2) , weight) | (n1 , n2 , weight)) } }
};
}
