macro_rules! deps {
    () => {
        Directed!();
        Create!();
        StableGraph!();
    };
}

macro_rules! impl_807 {
    () => {
        deps!();
        impl < N , E > StableGraph < N , E , Directed > { # [doc = " Create a new `StableGraph` with directed edges."] # [doc = ""] # [doc = " This is a convenience method. See `StableGraph::with_capacity`"] # [doc = " or `StableGraph::default` for a constructor that is generic in all the"] # [doc = " type parameters of `StableGraph`."] pub fn new () -> Self { Self :: with_capacity (0 , 0) } }
    };
}

impl_807!()