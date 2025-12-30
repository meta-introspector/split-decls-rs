// Generated macro for impl_1308 (impl)
macro_rules! Depcrate_matrix_graphimpl_1308 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1308"}
// Dependencies: {}
impl < 'a , N : 'a , Ix , S : BuildHasher > NodeReferences < 'a , N , Ix , S > { fn new (nodes : & 'a IdStorage < N , S >) -> Self { NodeReferences { nodes , iter : nodes . iter_ids () , ix : PhantomData , } } }
};
}
