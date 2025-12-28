macro_rules! deps {
    () => {
        MatrixGraph!();
        Undirected!();
        DefaultIx!();
    };
}

macro_rules! UnMatrix {
    () => {
        deps!();
        # [doc = " A `MatrixGraph` with undirected edges."] pub type UnMatrix < N , E , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , Null = Option < E > , Ix = DefaultIx , > = MatrixGraph < N , E , S , Undirected , Null , Ix > ;
    };
}

UnMatrix!();