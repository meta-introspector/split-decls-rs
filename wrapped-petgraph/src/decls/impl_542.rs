macro_rules! deps {
    () => {
        IndexType!();
        Csr!();
        NodeIndex!();
        EdgeType!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > IndexMut < NodeIndex < Ix > > for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , ix : NodeIndex < Ix >) -> & mut N { & mut self . node_weights [ix . index ()] } }
    };
}

impl_542!();