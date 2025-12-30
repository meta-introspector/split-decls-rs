// Generated macro for impl_27 (impl)
macro_rules! Depcrate_assert_dep_graphimpl_27 {
() => {
// Module: crate::assert_dep_graph
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > dot :: Labeller < 'a > for GraphvizDepGraph { type Node = DepKind ; type Edge = (DepKind , DepKind) ; fn graph_id (& self) -> dot :: Id < '_ > { dot :: Id :: new ("DependencyGraph") . unwrap () } fn node_id (& self , n : & DepKind) -> dot :: Id < '_ > { let s : String = format ! ("{n:?}") . chars () . map (| c | if c == '_' || c . is_alphanumeric () { c } else { '_' }) . collect () ; debug ! ("n={:?} s={:?}" , n , s) ; dot :: Id :: new (s) . unwrap () } fn node_label (& self , n : & DepKind) -> dot :: LabelText < '_ > { dot :: LabelText :: label (format ! ("{n:?}")) } }
};
}
