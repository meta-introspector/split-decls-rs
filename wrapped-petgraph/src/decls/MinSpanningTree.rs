macro_rules! deps {
    () => {
        Node!();
        MinScored!();
        Edge!();
        NodeReferences!();
        UnionFind!();
    };
}

macro_rules! MinSpanningTree {
    () => {
        deps!();
        # [doc = " An iterator producing a minimum spanning forest of a graph."] # [doc = " It will first iterate all Node elements from original graph,"] # [doc = " then iterate Edge elements from computed minimum spanning forest."] # [derive (Debug , Clone)] pub struct MinSpanningTree < G > where G : Data + IntoNodeReferences , { graph : G , node_ids : Option < G :: NodeReferences > , subgraphs : UnionFind < usize > , # [allow (clippy :: type_complexity)] sort_edges : BinaryHeap < MinScored < G :: EdgeWeight , (G :: NodeId , G :: NodeId) > > , node_map : HashMap < usize , usize > , node_count : usize , }
    };
}

MinSpanningTree!()