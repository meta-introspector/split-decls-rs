// Generated macro for impl_673 (impl)
macro_rules! Depcrate_view_crate_graphimpl_673 {
() => {
// Module: crate::view_crate_graph
// Provides: {"impl_673"}
// Dependencies: {}
impl < 'a > dot :: Labeller < 'a , Crate , Edge < 'a > > for DotCrateGraph < '_ > { fn graph_id (& 'a self) -> Id < 'a > { Id :: new ("rust_analyzer_crate_graph") . unwrap () } fn node_id (& 'a self , n : & Crate) -> Id < 'a > { let id = n . as_id () . index () ; Id :: new (format ! ("_{id:?}")) . unwrap () } fn node_shape (& 'a self , _node : & Crate) -> Option < LabelText < 'a > > { Some (LabelText :: LabelStr ("box" . into ())) } fn node_label (& 'a self , n : & Crate) -> LabelText < 'a > { let name = self . crates_to_render [n] . 1 . display_name . as_ref () . map_or ("(unnamed crate)" , | name | name . as_str ()) ; LabelText :: LabelStr (name . into ()) } }
};
}
