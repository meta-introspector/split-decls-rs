macro_rules! deps {
    () => {
        MatrixGraph!();
        IndexType!();
        NodeIndex!();
        Nullable!();
        EdgeType!();
    };
}

macro_rules! impl_1026 {
    () => {
        deps!();
        # [doc = " Index the `MatrixGraph` by `NodeIndex` pair to access edge weights."] # [doc = ""] # [doc = " Also available with indexing syntax: `&graph[e]`."] # [doc = ""] # [doc = " **Panics** if no edge exists between `a` and `b`."] impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Index < (NodeIndex < Ix > , NodeIndex < Ix >) > for MatrixGraph < N , E , S , Ty , Null , Ix > { type Output = E ; fn index (& self , (ax , bx) : (NodeIndex < Ix > , NodeIndex < Ix >)) -> & E { self . edge_weight (ax , bx) } }
    };
}

impl_1026!()