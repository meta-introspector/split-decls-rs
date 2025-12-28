macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        Csr!();
        NodeIndex!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Index < NodeIndex < Ix > > for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = N ; fn index (& self , ix : NodeIndex < Ix >) -> & N { & self . node_weights [ix . index ()] } }
    };
}

impl_541!()