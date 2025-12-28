macro_rules! deps {
    () => {
        MatrixGraph!();
        Directed!();
        Create!();
    };
}

macro_rules! impl_1020 {
    () => {
        deps!();
        impl < N , E , S : BuildHasher + Default > MatrixGraph < N , E , S , Directed > { # [doc = " Create a new `MatrixGraph` with directed edges."] # [doc = ""] # [doc = " This is a convenience method. Use `MatrixGraph::with_capacity` or `MatrixGraph::default` for"] # [doc = " a constructor that is generic in all the type parameters of `MatrixGraph`."] pub fn new () -> Self { MatrixGraph :: default () } }
    };
}

impl_1020!();