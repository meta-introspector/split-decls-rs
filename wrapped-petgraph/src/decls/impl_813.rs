macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        StableGraph!();
        EdgeIndex!();
    };
}

macro_rules! impl_813 {
    () => {
        deps!();
        # [doc = " Index the `StableGraph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > Index < EdgeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = E ; fn index (& self , index : EdgeIndex < Ix >) -> & E { self . edge_weight (index) . unwrap () } }
    };
}

impl_813!()