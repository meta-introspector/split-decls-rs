macro_rules! deps {
    () => {
        UnGraph!();
        WalkNeighbors!();
        Graph!();
        GraphError!();
        EdgeReference!();
        NodeIndex!();
        Node!();
        GraphIndex!();
        EdgeWeightsMut!();
        DefaultIx!();
        Neighbors!();
        NodeReferences!();
        IndexType!();
        EdgeIndex!();
        EdgesConnecting!();
        EdgeReferences!();
        NodeIndices!();
        NodeWeightsMut!();
        EdgeIndices!();
        Edges!();
        DiGraph!();
        Externals!();
        Edge!();
        Frozen!();
    };
}

macro_rules! graph {
    () => {
        deps!();
        # [doc = " `Graph<N, E, Ty, Ix>` is a graph datastructure using an adjacency list representation."] pub mod graph { pub use crate :: graph_impl :: { DefaultIx , DiGraph , Edge , EdgeIndex , EdgeIndices , EdgeReference , EdgeReferences , EdgeWeightsMut , Edges , EdgesConnecting , Externals , Frozen , Graph , GraphError , GraphIndex , IndexType , Neighbors , Node , NodeIndex , NodeIndices , NodeReferences , NodeWeightsMut , UnGraph , WalkNeighbors , edge_index , node_index , } ; }
    };
}

graph!()