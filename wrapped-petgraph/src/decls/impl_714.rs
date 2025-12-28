macro_rules! deps {
    () => {
        EdgeIndex!();
        IndexType!();
        EdgeType!();
        Graph!();
    };
}

macro_rules! impl_714 {
    () => {
        deps!();
        # [doc = " Index the `Graph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > Index < EdgeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = E ; fn index (& self , index : EdgeIndex < Ix >) -> & E { & self . edges [index . index ()] . weight } }
    };
}

impl_714!();