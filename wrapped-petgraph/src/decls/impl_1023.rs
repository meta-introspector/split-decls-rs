macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        NodeIndex!();
        MatrixGraph!();
        Nullable!();
    };
}

macro_rules! impl_1023 {
    () => {
        deps!();
        # [doc = " Index the `MatrixGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IndexMut < NodeIndex < Ix > > for MatrixGraph < N , E , S , Ty , Null , Ix > { fn index_mut (& mut self , ax : NodeIndex < Ix >) -> & mut N { self . node_weight_mut (ax) } }
    };
}

impl_1023!()