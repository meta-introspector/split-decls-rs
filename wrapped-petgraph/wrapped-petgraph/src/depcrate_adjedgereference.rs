// Generated macro for EdgeReference (struct)
macro_rules! Depcrate_adjEdgeReference {
() => {
// Module: crate::adj
// Provides: {"EdgeReference"}
// Dependencies: {}
# [doc = " A reference to an edge of the graph."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd)] pub struct EdgeReference < 'a , E , Ix : IndexType > { # [doc = " index of the edge"] id : EdgeIndex < Ix > , # [doc = " a reference to the corresponding item in the adjacency list"] edge : & 'a WSuc < E , Ix > , }
};
}
