macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
        NodeTrait!();
    };
}

macro_rules! impl_1048 {
    () => {
        deps!();
        # [doc = " `Arbitrary` for `GraphMap` creates a graph by selecting a node count"] # [doc = " and a probability for each possible edge to exist."] # [doc = ""] # [doc = " The result will be simple graph or digraph, self loops"] # [doc = " possible, no parallel edges."] # [doc = ""] # [doc = " The exact properties of the produced graph is subject to change."] # [doc = ""] # [doc = " Requires crate features `\"quickcheck\"` and `\"graphmap\"`"] # [cfg (feature = "graphmap")] impl < N , E , Ty > Arbitrary for GraphMap < N , E , Ty > where N : NodeTrait + Arbitrary , E : Arbitrary , Ty : EdgeType + Clone + Send + 'static , { fn arbitrary < G : Gen > (g : & mut G) -> Self { let nodes = usize :: arbitrary (g) ; if nodes == 0 { return GraphMap :: with_capacity (0 , 0) ; } let mut nodes = (0 .. nodes) . map (| _ | N :: arbitrary (g)) . collect :: < Vec < _ > > () ; nodes . sort () ; nodes . dedup () ; let edge_prob = random_01 (g) * random_01 (g) ; let edges = ((nodes . len () as f64) . powi (2) * edge_prob) as usize ; let mut gr = GraphMap :: with_capacity (nodes . len () , edges) ; for & node in & nodes { gr . add_node (node) ; } for (index , & i) in nodes . iter () . enumerate () { let js = if Ty :: is_directed () { & nodes [..] } else { & nodes [index ..] } ; for & j in js { let p : f64 = random_01 (g) ; if p <= edge_prob { gr . add_edge (i , j , E :: arbitrary (g)) ; } } } gr } }
    };
}

impl_1048!();