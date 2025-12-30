// Generated macro for impl_966 (impl)
macro_rules! Depcrate_graph_implimpl_966 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_966"}
// Dependencies: {}
impl < E , Ix > EdgesWalkerMut < '_ , E , Ix > where Ix : IndexType , { fn next_edge (& mut self) -> Option < & mut Edge < E , Ix > > { self . next () . map (| t | t . 1) } fn next (& mut self) -> Option < (EdgeIndex < Ix > , & mut Edge < E , Ix >) > { let this_index = self . next ; let k = self . dir . index () ; match self . edges . get_mut (self . next . index ()) { None => None , Some (edge) => { self . next = edge . next [k] ; Some ((this_index , edge)) } } } }
};
}
