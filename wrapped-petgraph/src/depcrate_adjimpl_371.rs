// Generated macro for impl_371 (impl)
macro_rules! Depcrate_adjimpl_371 {
() => {
// Module: crate::adj
// Provides: {"impl_371"}
// Dependencies: {}
impl < E , Ix : IndexType > DataMapMut for List < E , Ix > { fn node_weight_mut (& mut self , n : Self :: NodeId) -> Option < & mut () > { if n . index () < self . suc . len () { let b = Box :: new (()) ; Some (Box :: leak (b)) } else { None } } # [doc = " Accesses the weight of edge `e`"] # [doc = ""] # [doc = " Computes in **O(1)**"] fn edge_weight_mut (& mut self , e : EdgeIndex < Ix >) -> Option < & mut E > { self . get_edge_mut (e) . map (| x | & mut x . weight) } }
};
}
