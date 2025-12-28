macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        StableGraph!();
    };
}

macro_rules! impl_1047 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] # [doc = " `Arbitrary` for `StableGraph` creates a graph by selecting a node count"] # [doc = " and a probability for each possible edge to exist."] # [doc = ""] # [doc = " The result will be simple graph or digraph, with possible"] # [doc = " self loops, no parallel edges."] # [doc = ""] # [doc = " The exact properties of the produced graph is subject to change."] # [doc = ""] # [doc = " Requires crate features `\"quickcheck\"` and `\"stable_graph\"`"] impl < N , E , Ty , Ix > Arbitrary for StableGraph < N , E , Ty , Ix > where N : Arbitrary , E : Arbitrary , Ty : EdgeType + Send + 'static , Ix : IndexType + Send , { fn arbitrary < G : Gen > (g : & mut G) -> Self { let nodes = usize :: arbitrary (g) ; if nodes == 0 { return StableGraph :: with_capacity (0 , 0) ; } let edge_prob = random_01 (g) * random_01 (g) ; let edges = ((nodes as f64) . powi (2) * edge_prob) as usize ; let mut gr = StableGraph :: with_capacity (nodes , edges) ; for _ in 0 .. nodes { gr . add_node (N :: arbitrary (g)) ; } for i in 0 .. gr . node_count () { for j in 0 .. gr . node_count () { let i = node_index (i) ; let j = node_index (j) ; if ! gr . is_directed () && i > j { continue ; } let p : f64 = random_01 (g) ; if p <= edge_prob { gr . add_edge (i , j , E :: arbitrary (g)) ; } } } if bool :: arbitrary (g) { let n = u8 :: arbitrary (g) % (gr . node_count () as u8) ; for _ in 0 .. n { let ni = node_index (usize :: arbitrary (g) % gr . node_bound ()) ; if gr . node_weight (ni) . is_some () { gr . remove_node (ni) ; } } } gr } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { let self_ = self . clone () ; Box :: new ((0 .. 2) . filter_map (move | x | { let gr = self_ . filter_map (| i , w | { if i . index () % 2 == x { Some (w . clone ()) } else { None } } , | _ , w | Some (w . clone ()) ,) ; if gr . node_count () < self_ . node_count () { Some (gr) } else { None } })) } }
    };
}

impl_1047!()