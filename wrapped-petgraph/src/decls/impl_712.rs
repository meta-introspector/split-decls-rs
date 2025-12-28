macro_rules! deps {
    () => {
        IndexType!();
        Graph!();
        NodeIndex!();
        EdgeType!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        # [doc = " Index the `Graph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > Index < NodeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = N ; fn index (& self , index : NodeIndex < Ix >) -> & N { & self . nodes [index . index ()] . weight } }
    };
}

impl_712!();