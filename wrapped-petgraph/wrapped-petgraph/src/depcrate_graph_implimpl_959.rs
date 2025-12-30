// Generated macro for impl_959 (impl)
macro_rules! Depcrate_graph_implimpl_959 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_959"}
// Dependencies: {}
impl < 'a , N : 'a , Ty , Ix > Iterator for Externals < 'a , N , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < NodeIndex < Ix > > { let k = self . dir . index () ; loop { match self . iter . next () { None => return None , Some ((index , node)) => { if node . next [k] == EdgeIndex :: end () && (Ty :: is_directed () || node . next [1 - k] == EdgeIndex :: end ()) { return Some (NodeIndex :: new (index)) ; } else { continue ; } } } } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}
