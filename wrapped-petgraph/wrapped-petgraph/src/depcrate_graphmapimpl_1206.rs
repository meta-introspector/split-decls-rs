// Generated macro for impl_1206 (impl)
macro_rules! Depcrate_graphmapimpl_1206 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1206"}
// Dependencies: {}
impl < 'a , N , E , Ty > Iterator for AllEdges < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { type Item = (N , N , & 'a E) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (& (a , b) , v) | (a , b , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn count (self) -> usize { self . inner . count () } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . inner . nth (n) . map (| (& (n1 , n2) , weight) | (n1 , n2 , weight)) } fn last (self) -> Option < Self :: Item > { self . inner . last () . map (| (& (n1 , n2) , weight) | (n1 , n2 , weight)) } }
};
}
