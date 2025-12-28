macro_rules! deps {
    () => {
        EdgeType!();
        NodeIndex!();
        StableGraph!();
        IndexType!();
    };
}

macro_rules! impl_811 {
    () => {
        deps!();
        # [doc = " Index the `StableGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > Index < NodeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = N ; fn index (& self , index : NodeIndex < Ix >) -> & N { self . node_weight (index) . unwrap () } }
    };
}

impl_811!()