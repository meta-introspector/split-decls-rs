// Generated macro for EdgeIndex (struct)
macro_rules! Depcrate_adjEdgeIndex {
() => {
// Module: crate::adj
// Provides: {"EdgeIndex"}
// Dependencies: {}
# [doc = " Adjacency list edge index type, a pair of integers."] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] pub struct EdgeIndex < Ix = DefaultIx > where Ix : IndexType , { # [doc = " Source of the edge."] from : NodeIndex < Ix > , # [doc = " Index of the successor in the successor list."] successor_index : usize , }
};
}
