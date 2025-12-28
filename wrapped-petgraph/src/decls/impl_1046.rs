macro_rules! deps {
    () => {
        Graph!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_1046 {
    () => {
        deps!();
        # [doc = " `Arbitrary` for `Graph` creates a graph by selecting a node count"] # [doc = " and a probability for each possible edge to exist."] # [doc = ""] # [doc = " The result will be simple graph or digraph, self loops"] # [doc = " possible, no parallel edges."] # [doc = ""] # [doc = " The exact properties of the produced graph is subject to change."] # [doc = ""] # [doc = " Requires crate feature `\"quickcheck\"`"] impl < N , E , Ty , Ix > Arbitrary for Graph < N , E , Ty , Ix > where N : Arbitrary , E : Arbitrary , Ty : EdgeType + Send + 'static , Ix : IndexType + Send , { fn arbitrary < G : Gen > (g : & mut G) -> Self { let nodes = usize :: arbitrary (g) ; if nodes == 0 { return Graph :: with_capacity (0 , 0) ; } let edge_prob = random_01 (g) * random_01 (g) ; let edges = ((nodes as f64) . powi (2) * edge_prob) as usize ; let mut gr = Graph :: with_capacity (nodes , edges) ; for _ in 0 .. nodes { gr . add_node (N :: arbitrary (g)) ; } for i in gr . node_indices () { for j in gr . node_indices () { if ! gr . is_directed () && i > j { continue ; } let p : f64 = random_01 (g) ; if p <= edge_prob { gr . add_edge (i , j , E :: arbitrary (g)) ; } } } gr } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { let self_ = self . clone () ; Box :: new ((0 .. 2) . filter_map (move | x | { let gr = self_ . filter_map (| i , w | { if i . index () % 2 == x { Some (w . clone ()) } else { None } } , | _ , w | Some (w . clone ()) ,) ; if gr . node_count () < self_ . node_count () { Some (gr) } else { None } })) } }
    };
}

impl_1046!();