macro_rules! deps {
    () => {
        MatrixGraph!();
        DefaultIx!();
        Directed!();
    };
}

macro_rules! DiMatrix {
    () => {
        deps!();
        # [doc = " A `MatrixGraph` with directed edges."] pub type DiMatrix < N , E , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , Null = Option < E > , Ix = DefaultIx , > = MatrixGraph < N , E , S , Directed , Null , Ix > ;
    };
}

DiMatrix!();