macro_rules! deps {
    () => {
        NodeIndices!();
        NodeReferences!();
        UnGraph!();
        WalkNeighbors!();
        Edges!();
        NodeIndex!();
        EdgeIndex!();
        Edge!();
        NodeWeightsMut!();
        IndexType!();
        Externals!();
        EdgeIndices!();
        GraphError!();
        EdgeReferences!();
        Node!();
        Neighbors!();
        EdgeWeightsMut!();
        DiGraph!();
        DefaultIx!();
        Frozen!();
        EdgesConnecting!();
        Graph!();
        EdgeReference!();
        GraphIndex!();
    };
}

macro_rules! graph {
    () => {
        deps!();
        # [doc = " `Graph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation."] pub mod graph { pub use crate :: graph_impl :: { DefaultIx , DiGraph , Edge , EdgeIndex , EdgeIndices , EdgeReference , EdgeReferences , EdgeWeightsMut , Edges , EdgesConnecting , Externals , Frozen , Graph , GraphError , GraphIndex , IndexType , Neighbors , Node , NodeIndex , NodeIndices , NodeReferences , NodeWeightsMut , UnGraph , WalkNeighbors , edge_index , node_index , } ; }
    };
}

graph!();