// Generated macro for impl_1226 (impl)
macro_rules! Depcrate_graphmapimpl_1226 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1226"}
// Dependencies: {}
impl < 'a , N , E , Ty > Iterator for NodeIdentifiers < 'a , N , E , Ty > where N : 'a + NodeTrait , E : 'a , Ty : EdgeType , { type Item = N ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (& n , _) | n) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
