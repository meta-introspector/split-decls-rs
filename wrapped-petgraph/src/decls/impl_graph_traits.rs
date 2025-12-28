macro_rules! deps {
    () => {
        Direction!();
        NodeReferences!();
        Cycle!();
        NodeIndex!();
        NeighborsDirected!();
        EdgesDirected!();
        Acyclic!();
        Neighbors!();
        NodeIdentifiers!();
        IndexType!();
        EdgeRef!();
        NodeRef!();
        Edges!();
        EdgeReferences!();
    };
}

macro_rules! impl_graph_traits {
    () => {
        deps!();
        macro_rules ! impl_graph_traits { ($ graph_type : ident) => { impl < N , E , Ix : IndexType > Acyclic <$ graph_type < N , E , Ix >> { # [doc = " Remove an edge and return its edge weight, or None if it didn't exist."] # [doc = ""] # [doc = " Pass through to underlying graph."] pub fn remove_edge (& mut self , e : <$ graph_type < N , E , Ix > as GraphBase >:: EdgeId ,) -> Option < E > { self . graph . remove_edge (e) } # [doc = " Remove a node from the graph if it exists, and return its"] # [doc = " weight. If it doesn't exist in the graph, return None."] # [doc = ""] # [doc = " This updates the order in O(v) runtime and removes the node in"] # [doc = " the underlying graph."] pub fn remove_node (& mut self , n : <$ graph_type < N , E , Ix > as GraphBase >:: NodeId ,) -> Option < N > { self . order_map . remove_node (n , & self . graph) ; self . graph . remove_node (n) } } impl < N , E , Ix : IndexType > TryFrom <$ graph_type < N , E , Ix >> for Acyclic <$ graph_type < N , E , Ix >> { type Error = Cycle < NodeIndex < Ix >>; fn try_from (graph : $ graph_type < N , E , Ix >) -> Result < Self , Self :: Error > { let order_map = OrderMap :: try_from_graph (& graph) ?; let discovered = RefCell :: new (FixedBitSet :: with_capacity (graph . node_bound ())) ; let finished = RefCell :: new (FixedBitSet :: with_capacity (graph . node_bound ())) ; Ok (Self { graph , order_map , discovered , finished , }) } } impl <'a , N , E , Ix : IndexType > IntoEdgeReferences for &'a Acyclic <$ graph_type < N , E , Ix >> { type EdgeRef = <&'a $ graph_type < N , E , Ix > as IntoEdgeReferences >:: EdgeRef ; type EdgeReferences = <&'a $ graph_type < N , E , Ix > as IntoEdgeReferences >:: EdgeReferences ; fn edge_references (self) -> Self :: EdgeReferences { self . inner () . edge_references () } } impl <'a , N , E , Ix : IndexType > IntoEdges for &'a Acyclic <$ graph_type < N , E , Ix >> { type Edges = <&'a $ graph_type < N , E , Ix > as IntoEdges >:: Edges ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . inner () . edges (a) } } impl <'a , N , E , Ix : IndexType > IntoEdgesDirected for &'a Acyclic <$ graph_type < N , E , Ix >> { type EdgesDirected = <&'a $ graph_type < N , E , Ix > as IntoEdgesDirected >:: EdgesDirected ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { self . inner () . edges_directed (a , dir) } } impl <'a , N , E , Ix : IndexType > IntoNeighbors for &'a Acyclic <$ graph_type < N , E , Ix >> { type Neighbors = <&'a $ graph_type < N , E , Ix > as IntoNeighbors >:: Neighbors ; fn neighbors (self , a : Self :: NodeId) -> Self :: Neighbors { self . inner () . neighbors (a) } } impl <'a , N , E , Ix : IndexType > IntoNeighborsDirected for &'a Acyclic <$ graph_type < N , E , Ix >> { type NeighborsDirected = <&'a $ graph_type < N , E , Ix > as IntoNeighborsDirected >:: NeighborsDirected ; fn neighbors_directed (self , n : Self :: NodeId , d : Direction) -> Self :: NeighborsDirected { self . inner () . neighbors_directed (n , d) } } impl <'a , N , E , Ix : IndexType > IntoNodeIdentifiers for &'a Acyclic <$ graph_type < N , E , Ix >> { type NodeIdentifiers = <&'a $ graph_type < N , E , Ix > as IntoNodeIdentifiers >:: NodeIdentifiers ; fn node_identifiers (self) -> Self :: NodeIdentifiers { self . inner () . node_identifiers () } } impl <'a , N , E , Ix : IndexType > IntoNodeReferences for &'a Acyclic <$ graph_type < N , E , Ix >> { type NodeRef = <&'a $ graph_type < N , E , Ix > as IntoNodeReferences >:: NodeRef ; type NodeReferences = <&'a $ graph_type < N , E , Ix > as IntoNodeReferences >:: NodeReferences ; fn node_references (self) -> Self :: NodeReferences { self . inner () . node_references () } } } ; }
    };
}

impl_graph_traits!();