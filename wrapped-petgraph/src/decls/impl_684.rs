macro_rules! deps {
    () => {
        Undirected!();
        Graph!();
        Create!();
    };
}

macro_rules! impl_684 {
    () => {
        deps!();
        impl < N , E > Graph < N , E , Undirected > { # [doc = " Create a new `Graph` with undirected edges."] # [doc = ""] # [doc = " This is a convenience method. Use `Graph::with_capacity` or `Graph::default` for"] # [doc = " a constructor that is generic in all the type parameters of `Graph`."] pub fn new_undirected () -> Self { Graph { nodes : Vec :: new () , edges : Vec :: new () , ty : PhantomData , } } }
    };
}

impl_684!()