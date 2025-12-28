macro_rules! deps {
    () => {
        Graph!();
        NodeIndex!();
        EdgeType!();
        Generator!();
    };
}

macro_rules! impl_594 {
    () => {
        deps!();
        impl < Ty : EdgeType > Generator < Ty > { # [doc = " Generate all possible graphs of a particular number of nodes."] # [doc = ""] # [doc = " All permutations are generated, so the graphs are not unique down to isomorphism."] # [doc = ""] # [doc = " For a graph of *k* nodes there are *e = k²* possible edges and"] # [doc = " *2<sup>k<sup>2</sup></sup>* graphs."] pub fn all (nodes : usize , allow_selfloops : bool) -> Self { let scale = if Ty :: is_directed () { 1 } else { 2 } ; let nedges = if allow_selfloops { (nodes * nodes - nodes) / scale + nodes } else { (nodes * nodes) / scale - nodes } ; assert ! (nedges < 64) ; Generator { acyclic : false , selfloops : allow_selfloops , nodes , nedges , bits : ! 0 , g : Graph :: with_capacity (nodes , nedges) , } } fn state_to_graph (& mut self) -> & Graph < () , () , Ty > { self . g . clear () ; for _ in 0 .. self . nodes { self . g . add_node (()) ; } let mut bit = 0 ; for i in 0 .. self . nodes { let start = if self . acyclic || ! self . g . is_directed () { i } else { 0 } ; for j in start .. self . nodes { if i == j && ! self . selfloops { continue ; } if self . bits & (1u64 << bit) != 0 { self . g . add_edge (NodeIndex :: new (i) , NodeIndex :: new (j) , ()) ; } bit += 1 ; } } & self . g } pub fn next_ref (& mut self) -> Option < & Graph < () , () , Ty > > { if self . bits == ! 0 { self . bits = 0 ; } else { self . bits += 1 ; if self . bits >= 1u64 << self . nedges { return None ; } } Some (self . state_to_graph ()) } }
    };
}

impl_594!();