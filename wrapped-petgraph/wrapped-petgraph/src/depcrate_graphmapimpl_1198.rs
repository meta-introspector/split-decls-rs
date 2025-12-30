// Generated macro for impl_1198 (impl)
macro_rules! Depcrate_graphmapimpl_1198 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1198"}
// Dependencies: {}
impl < N , Ty > Iterator for Neighbors < '_ , N , Ty > where N : NodeTrait , Ty : EdgeType , { type Item = N ; fn next (& mut self) -> Option < N > { if Ty :: is_directed () { (& mut self . iter) . filter_map (| & (n , dir) | if dir == Outgoing { Some (n) } else { None }) . next () } else { self . iter . next () . map (| & (n , _) | n) } } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . iter . size_hint () ; if Ty :: is_directed () { (0 , upper) } else { (lower , upper) } } }
};
}
