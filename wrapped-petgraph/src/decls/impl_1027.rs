macro_rules! deps {
    () => {
        Nullable!();
        IndexType!();
        EdgeType!();
        MatrixGraph!();
        NodeIndex!();
    };
}

macro_rules! impl_1027 {
    () => {
        deps!();
        # [doc = " Index the `MatrixGraph` by `NodeIndex` pair to access edge weights."] # [doc = ""] # [doc = " Also available with indexing syntax: `&mut graph[e]`."] # [doc = ""] # [doc = " **Panics** if no edge exists between `a` and `b`."] impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IndexMut < (NodeIndex < Ix > , NodeIndex < Ix >) > for MatrixGraph < N , E , S , Ty , Null , Ix > { fn index_mut (& mut self , (ax , bx) : (NodeIndex < Ix > , NodeIndex < Ix >)) -> & mut E { self . edge_weight_mut (ax , bx) } }
    };
}

impl_1027!()