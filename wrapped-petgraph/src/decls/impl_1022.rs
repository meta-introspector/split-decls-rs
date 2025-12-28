macro_rules! deps {
    () => {
        MatrixGraph!();
        NodeIndex!();
        IndexType!();
        Nullable!();
        EdgeType!();
    };
}

macro_rules! impl_1022 {
    () => {
        deps!();
        # [doc = " Index the `MatrixGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Index < NodeIndex < Ix > > for MatrixGraph < N , E , S , Ty , Null , Ix > { type Output = N ; fn index (& self , ax : NodeIndex < Ix >) -> & N { self . node_weight (ax) } }
    };
}

impl_1022!();