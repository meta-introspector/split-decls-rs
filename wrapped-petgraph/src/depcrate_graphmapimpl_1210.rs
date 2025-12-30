// Generated macro for impl_1210 (impl)
macro_rules! Depcrate_graphmapimpl_1210 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1210"}
// Dependencies: {}
impl < 'a , N , E , Ty > DoubleEndedIterator for AllEdgesMut < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (& (n1 , n2) , weight) | (n1 , n2 , weight)) } }
};
}
