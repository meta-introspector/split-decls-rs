macro_rules! deps {
    () => {
        Create!();
        IndexType!();
        Csr!();
        EdgeType!();
    };
}

macro_rules! impl_519 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [doc = " Create an empty `Csr`."] pub fn new () -> Self { Csr { column : vec ! [] , edges : vec ! [] , row : vec ! [0 ; 1] , node_weights : vec ! [] , edge_count : 0 , ty : PhantomData , } } # [doc = " Create a new `Csr` with `n` nodes. `N` must implement [`Default`] for the weight of each node."] # [doc = ""] # [doc = " [`Default`]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html"] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use petgraph::csr::Csr;"] # [doc = " use petgraph::prelude::*;"] # [doc = ""] # [doc = " let graph = Csr::<u8,()>::with_nodes(5);"] # [doc = " assert_eq!(graph.node_count(),5);"] # [doc = " assert_eq!(graph.edge_count(),0);"] # [doc = ""] # [doc = " assert_eq!(graph[0],0);"] # [doc = " assert_eq!(graph[4],0);"] # [doc = " ```"] pub fn with_nodes (n : usize) -> Self where N : Default , { Csr { column : Vec :: new () , edges : Vec :: new () , row : vec ! [0 ; n + 1] , node_weights : (0 .. n) . map (| _ | N :: default ()) . collect () , edge_count : 0 , ty : PhantomData , } } }
    };
}

impl_519!();