// Generated macro for impl_1311 (impl)
macro_rules! Depcrate_matrix_graphimpl_1311 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1311"}
// Dependencies: {}
impl < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > EdgeReferences < 'a , Ty , Null , Ix > { fn new (node_adjacencies : & 'a [Null] , node_capacity : usize) -> Self { EdgeReferences { row : 0 , column : 0 , node_adjacencies , node_capacity , ty : PhantomData , ix : PhantomData , } } }
};
}
