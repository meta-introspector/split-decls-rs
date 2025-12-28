macro_rules! deps {
    () => {
        Node!();
        NodeRef!();
        Edge!();
        MinScored!();
        NodeReferences!();
    };
}

macro_rules! MinSpanningTreePrim {
    () => {
        deps!();
        # [doc = " An iterator producing a minimum spanning tree of a graph."] # [doc = " It will first iterate all Node elements from original graph,"] # [doc = " then iterate Edge elements from computed minimum spanning tree."] # [derive (Debug , Clone)] pub struct MinSpanningTreePrim < G > where G : IntoNodeReferences , { graph : G , node_ids : Option < G :: NodeReferences > , node_map : HashMap < usize , usize > , node_count : usize , # [allow (clippy :: type_complexity)] sort_edges : BinaryHeap < MinScored < G :: EdgeWeight , (G :: NodeId , G :: NodeId) > > , nodes_taken : HashSet < usize > , initial_node : Option < G :: NodeRef > , }
    };
}

MinSpanningTreePrim!();