// Generated macro for impl_26 (impl)
macro_rules! Depcrate_assert_dep_graphimpl_26 {
() => {
// Module: crate::assert_dep_graph
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a > dot :: GraphWalk < 'a > for GraphvizDepGraph { type Node = DepKind ; type Edge = (DepKind , DepKind) ; fn nodes (& self) -> dot :: Nodes < '_ , DepKind > { let nodes : Vec < _ > = self . 0 . iter () . cloned () . collect () ; nodes . into () } fn edges (& self) -> dot :: Edges < '_ , (DepKind , DepKind) > { self . 1 [..] . into () } fn source (& self , edge : & (DepKind , DepKind)) -> DepKind { edge . 0 } fn target (& self , edge : & (DepKind , DepKind)) -> DepKind { edge . 1 } }
};
}
