macro_rules! deps {
    () => {
        IndexType!();
        SerGraph!();
        EdgeType!();
        IntoSerializable!();
        EdgeProperty!();
        Graph!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > IntoSerializable for & 'a Graph < N , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { type Output = SerGraph < 'a , N , E , Ix > ; fn into_serializable (self) -> Self :: Output { SerGraph { nodes : & self . nodes , node_holes : & [] , edges : & self . edges , edge_property : EdgeProperty :: from (PhantomData :: < Ty >) , } } }
    };
}

impl_640!()