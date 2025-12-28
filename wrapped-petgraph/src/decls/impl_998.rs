macro_rules! deps {
    () => {
        EdgeType!();
        EdgeReferences!();
        Nullable!();
    };
}

macro_rules! impl_998 {
    () => {
        deps!();
        impl < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > EdgeReferences < 'a , Ty , Null , Ix > { fn new (node_adjacencies : & 'a [Null] , node_capacity : usize) -> Self { EdgeReferences { row : 0 , column : 0 , node_adjacencies , node_capacity , ty : PhantomData , ix : PhantomData , } } }
    };
}

impl_998!()