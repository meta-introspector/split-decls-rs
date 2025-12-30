// Generated macro for impl_1101 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1101 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1101"}
// Dependencies: {}
impl < N , E , Ty , Ix > fmt :: Debug for StableGraph < N , E , Ty , Ix > where N : fmt :: Debug , E : fmt :: Debug , Ty : EdgeType , Ix : IndexType , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let etype = if self . is_directed () { "Directed" } else { "Undirected" } ; let mut fmt_struct = f . debug_struct ("StableGraph") ; fmt_struct . field ("Ty" , & etype) ; fmt_struct . field ("node_count" , & self . node_count) ; fmt_struct . field ("edge_count" , & self . edge_count) ; if self . g . edges . iter () . any (| e | e . weight . is_some ()) { fmt_struct . field ("edges" , & self . g . edges . iter () . filter (| e | e . weight . is_some ()) . map (| e | NoPretty ((e . source () . index () , e . target () . index ()))) . format (", ") ,) ; } if size_of :: < N > () != 0 { fmt_struct . field ("node weights" , & DebugMap (| | { self . g . nodes . iter () . map (| n | n . weight . as_ref ()) . enumerate () . filter_map (| (i , wo) | wo . map (move | w | (i , w))) }) ,) ; } if size_of :: < E > () != 0 { fmt_struct . field ("edge weights" , & DebugMap (| | { self . g . edges . iter () . map (| n | n . weight . as_ref ()) . enumerate () . filter_map (| (i , wo) | wo . map (move | w | (i , w))) }) ,) ; } fmt_struct . field ("free_node" , & self . free_node) ; fmt_struct . field ("free_edge" , & self . free_edge) ; fmt_struct . finish () } }
};
}
