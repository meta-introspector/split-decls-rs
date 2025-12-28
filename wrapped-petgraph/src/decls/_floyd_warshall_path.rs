macro_rules! deps {
    () => {
        BoundedMeasure!();
        NegativeCycle!();
        EdgeRef!();
    };
}

macro_rules! _floyd_warshall_path {
    () => {
        deps!();
        # [doc = " Helper that implements the floyd warshall routine, but paths are optional for memory overhead."] fn _floyd_warshall_path < G , F , K > (graph : G , mut edge_cost : F , m_dist : & mut Option < Vec < Vec < K > > > , m_prev : & mut Option < Vec < Vec < Option < usize > > > > ,) -> Result < () , NegativeCycle > where G : NodeCompactIndexable + IntoEdgeReferences + IntoNodeIdentifiers + GraphProp , G :: NodeId : Eq + Hash , F : FnMut (G :: EdgeRef) -> K , K : BoundedMeasure + Copy , { let num_of_nodes = graph . node_count () ; for edge in graph . edge_references () { let source = graph . to_index (edge . source ()) ; let target = graph . to_index (edge . target ()) ; let cost = edge_cost (edge) ; if is_greater (m_dist , source , target , cost) { set_object (m_dist , source , target , cost) ; set_object (m_prev , source , target , Some (source)) ; if ! graph . is_directed () { set_object (m_dist , target , source , cost) ; set_object (m_prev , target , source , Some (target)) ; } } } for node in graph . node_identifiers () { let index = graph . to_index (node) ; set_object (m_dist , index , index , K :: default ()) ; set_object (m_prev , index , index , Some (index)) ; } for k in 0 .. num_of_nodes { for i in 0 .. num_of_nodes { for j in 0 .. num_of_nodes { if let Some (dist) = m_dist { let (result , overflow) = dist [i] [k] . overflowing_add (dist [k] [j]) ; if ! overflow && dist [i] [j] > result { dist [i] [j] = result ; if let Some (prev) = m_prev { prev [i] [j] = prev [k] [j] ; } } } } } } for i in 0 .. num_of_nodes { if let Some (dist) = m_dist { if dist [i] [i] < K :: default () { return Err (NegativeCycle (())) ; } } } Ok (()) }
    };
}

_floyd_warshall_path!();