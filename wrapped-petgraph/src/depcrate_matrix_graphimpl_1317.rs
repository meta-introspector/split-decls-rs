// Generated macro for impl_1317 (impl)
macro_rules! Depcrate_matrix_graphimpl_1317 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1317"}
// Dependencies: {}
impl < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > Edges < 'a , Ty , Null , Ix > { fn on_columns (row : usize , node_adjacencies : & 'a [Null] , node_capacity : usize) -> Self { Edges { iter_direction : NeighborIterDirection :: Columns , node_adjacencies , node_capacity , row , column : 0 , ty : PhantomData , ix : PhantomData , } } fn on_rows (column : usize , node_adjacencies : & 'a [Null] , node_capacity : usize) -> Self { Edges { iter_direction : NeighborIterDirection :: Rows , node_adjacencies , node_capacity , row : 0 , column , ty : PhantomData , ix : PhantomData , } } }
};
}
