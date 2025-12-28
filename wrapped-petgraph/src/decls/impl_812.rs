macro_rules! deps {
    () => {
        IndexType!();
        NodeIndex!();
        EdgeType!();
        StableGraph!();
    };
}

macro_rules! impl_812 {
    () => {
        deps!();
        # [doc = " Index the `StableGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > IndexMut < NodeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : NodeIndex < Ix >) -> & mut N { self . node_weight_mut (index) . unwrap () } }
    };
}

impl_812!()