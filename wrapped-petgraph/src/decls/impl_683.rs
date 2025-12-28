macro_rules! deps {
    () => {
        Create!();
        Graph!();
        Directed!();
    };
}

macro_rules! impl_683 {
    () => {
        deps!();
        impl < N , E > Graph < N , E , Directed > { # [doc = " Create a new `Graph` with directed edges."] # [doc = ""] # [doc = " This is a convenience method. Use `Graph::with_capacity` or `Graph::default` for"] # [doc = " a constructor that is generic in all the type parameters of `Graph`."] pub fn new () -> Self { Graph { nodes : Vec :: new () , edges : Vec :: new () , ty : PhantomData , } } }
    };
}

impl_683!()