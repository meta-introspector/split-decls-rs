macro_rules! deps {
    () => {
        NeighborIterDirection!();
        EdgeType!();
        Nullable!();
        Edges!();
    };
}

macro_rules! impl_1004 {
    () => {
        deps!();
        impl < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > Edges < 'a , Ty , Null , Ix > { fn on_columns (row : usize , node_adjacencies : & 'a [Null] , node_capacity : usize) -> Self { Edges { iter_direction : NeighborIterDirection :: Columns , node_adjacencies , node_capacity , row , column : 0 , ty : PhantomData , ix : PhantomData , } } fn on_rows (column : usize , node_adjacencies : & 'a [Null] , node_capacity : usize) -> Self { Edges { iter_direction : NeighborIterDirection :: Rows , node_adjacencies , node_capacity , row : 0 , column , ty : PhantomData , ix : PhantomData , } } }
    };
}

impl_1004!()