// Generated macro for graph (module)
macro_rules! Depcrategraph {
() => {
// Module: crate
// Provides: {"graph"}
// Dependencies: {}
# [doc = " `Graph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation."] pub mod graph { pub use crate :: graph_impl :: { DefaultIx , DiGraph , Edge , EdgeIndex , EdgeIndices , EdgeReference , EdgeReferences , EdgeWeightsMut , Edges , EdgesConnecting , Externals , Frozen , Graph , GraphError , GraphIndex , IndexType , Neighbors , Node , NodeIndex , NodeIndices , NodeReferences , NodeWeightsMut , UnGraph , WalkNeighbors , edge_index , node_index , } ; }
};
}
