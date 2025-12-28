macro_rules! deps {
    () => {
        Graph!();
        IndexType!();
        EdgeType!();
        NodeIndex!();
    };
}

macro_rules! impl_713 {
    () => {
        deps!();
        # [doc = " Index the `Graph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > IndexMut < NodeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : NodeIndex < Ix >) -> & mut N { & mut self . nodes [index . index ()] . weight } }
    };
}

impl_713!()