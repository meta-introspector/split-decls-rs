// Generated macro for impl_39 (impl)
macro_rules! Depcrate_testsimpl_39 {
() => {
// Module: crate::tests
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > GraphWalk < 'a > for LabelledGraphWithEscStrs { type Node = Node ; type Edge = & 'a Edge ; fn nodes (& 'a self) -> Nodes < 'a , Node > { self . graph . nodes () } fn edges (& 'a self) -> Edges < 'a , & 'a Edge > { self . graph . edges () } fn source (& 'a self , edge : & & 'a Edge) -> Node { edge . from } fn target (& 'a self , edge : & & 'a Edge) -> Node { edge . to } }
};
}
