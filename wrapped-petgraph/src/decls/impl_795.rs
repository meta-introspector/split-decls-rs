macro_rules! deps {
    () => {
        StableGraph!();
        DeserStableGraph!();
        Node!();
        Graph!();
        EdgeIndex!();
        NodeIndex!();
        FromDeserialized!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_795 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > FromDeserialized for StableGraph < N , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { type Input = DeserStableGraph < N , E , Ix > ; fn from_deserialized < E2 > (input : Self :: Input) -> Result < Self , E2 > where E2 : Error , { let ty = PhantomData :: < Ty > :: from_deserialized (input . edge_property) ? ; let node_holes = input . node_holes ; let edges = input . edges ; if edges . len () >= < Ix as IndexType > :: max () . index () { Err (invalid_length_err :: < Ix , _ > ("edge" , edges . len ())) ? } let total_nodes = input . nodes . len () + node_holes . len () ; let mut nodes = Vec :: with_capacity (total_nodes) ; let mut compact_nodes = input . nodes . into_iter () ; let mut node_pos = 0 ; for hole_pos in node_holes . iter () { let hole_pos = hole_pos . index () ; if ! (node_pos .. total_nodes) . contains (& hole_pos) { return Err (invalid_hole_err (hole_pos)) ; } nodes . extend (compact_nodes . by_ref () . take (hole_pos - node_pos)) ; nodes . push (Node { weight : None , next : [EdgeIndex :: end () ; 2] , }) ; node_pos = hole_pos + 1 ; debug_assert_eq ! (nodes . len () , node_pos) ; } nodes . extend (compact_nodes) ; if nodes . len () >= < Ix as IndexType > :: max () . index () { Err (invalid_length_err :: < Ix , _ > ("node" , nodes . len ())) ? } let node_bound = nodes . len () ; let mut sgr = StableGraph { g : Graph { nodes , edges , ty } , node_count : 0 , edge_count : 0 , free_edge : EdgeIndex :: end () , free_node : NodeIndex :: end () , } ; sgr . link_edges () . map_err (| i | invalid_node_err (i . index () , node_bound)) ? ; Ok (sgr) } }
    };
}

impl_795!()