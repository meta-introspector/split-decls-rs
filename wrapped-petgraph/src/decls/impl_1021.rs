macro_rules! deps {
    () => {
        Create!();
        Undirected!();
        MatrixGraph!();
    };
}

macro_rules! impl_1021 {
    () => {
        deps!();
        impl < N , E , S : BuildHasher + Default > MatrixGraph < N , E , S , Undirected > { # [doc = " Create a new `MatrixGraph` with undirected edges."] # [doc = ""] # [doc = " This is a convenience method. Use `MatrixGraph::with_capacity` or `MatrixGraph::default` for"] # [doc = " a constructor that is generic in all the type parameters of `MatrixGraph`."] pub fn new_undirected () -> Self { MatrixGraph :: default () } }
    };
}

impl_1021!();