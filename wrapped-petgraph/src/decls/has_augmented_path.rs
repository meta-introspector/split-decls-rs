macro_rules! deps {
    () => {
        PositiveMeasure!();
        Direction!();
        EdgeRef!();
    };
}

macro_rules! has_augmented_path {
    () => {
        deps!();
        # [doc = " Tells whether there is an augmented path in the graph"] fn has_augmented_path < G > (network : G , source : G :: NodeId , destination : G :: NodeId , edge_to : & mut [Option < G :: EdgeRef >] , flows : & [G :: EdgeWeight] ,) -> bool where G : NodeCount + IntoEdgesDirected + NodeIndexable + EdgeIndexable + Visitable , G :: EdgeWeight : Sub < Output = G :: EdgeWeight > + PositiveMeasure , { let mut visited = network . visit_map () ; let mut queue = VecDeque :: new () ; visited . visit (source) ; queue . push_back (source) ; while let Some (vertex) = queue . pop_front () { let out_edges = network . edges_directed (vertex , Direction :: Outgoing) ; let in_edges = network . edges_directed (vertex , Direction :: Incoming) ; for edge in out_edges . chain (in_edges) { let next = other_endpoint (& network , edge , vertex) ; let edge_index : usize = EdgeIndexable :: to_index (& network , edge . id ()) ; let residual_cap = residual_capacity (& network , edge , next , flows [edge_index]) ; if ! visited . is_visited (& next) && (residual_cap > G :: EdgeWeight :: zero ()) { visited . visit (next) ; edge_to [NodeIndexable :: to_index (& network , next)] = Some (edge) ; if destination == next { return true ; } queue . push_back (next) ; } } } false }
    };
}

has_augmented_path!()