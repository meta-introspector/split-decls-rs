macro_rules! deps {
    () => {
        EdgeIndex!();
        Graph!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        # [doc = " Index the `Graph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > IndexMut < EdgeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : EdgeIndex < Ix >) -> & mut E { & mut self . edges [index . index ()] . weight } }
    };
}

impl_715!();