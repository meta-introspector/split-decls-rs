macro_rules! deps {
    () => {
        EdgeIndex!();
        StableGraph!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_814 {
    () => {
        deps!();
        # [doc = " Index the `StableGraph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > IndexMut < EdgeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : EdgeIndex < Ix >) -> & mut E { self . edge_weight_mut (index) . unwrap () } }
    };
}

impl_814!();