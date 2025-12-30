// Generated macro for impl_865 (impl)
macro_rules! Depcrate_region_infer_graphvizimpl_865 {
() => {
// Module: crate::region_infer::graphviz
// Provides: {"impl_865"}
// Dependencies: {}
impl < 'a , 'this , 'tcx > dot :: Labeller < 'this > for SccConstraints < 'a , 'tcx > { type Node = ConstraintSccIndex ; type Edge = (ConstraintSccIndex , ConstraintSccIndex) ; fn graph_id (& 'this self) -> dot :: Id < 'this > { dot :: Id :: new ("RegionInferenceContext" . to_string ()) . unwrap () } fn node_id (& 'this self , n : & ConstraintSccIndex) -> dot :: Id < 'this > { dot :: Id :: new (format ! ("r{}" , n . index ())) . unwrap () } fn node_shape (& 'this self , _node : & ConstraintSccIndex) -> Option < dot :: LabelText < 'this > > { Some (dot :: LabelText :: LabelStr (Cow :: Borrowed ("box"))) } fn node_label (& 'this self , n : & ConstraintSccIndex) -> dot :: LabelText < 'this > { let nodes_str = self . nodes_per_scc [* n] . iter () . map (| n | render_region_vid (self . tcx , * n , self . regioncx)) . join (", ") ; dot :: LabelText :: LabelStr (format ! ("SCC({n}) = {{{nodes_str}}}" , n = n . as_usize ()) . into ()) } }
};
}
