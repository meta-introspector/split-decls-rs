// Generated macro for impl_904 (impl)
macro_rules! Depcrate_graph_impl_serializationimpl_904 {
() => {
// Module: crate::graph_impl::serialization
// Provides: {"impl_904"}
// Dependencies: {}
impl < Ty > From < PhantomData < Ty > > for EdgeProperty where Ty : EdgeType , { fn from (_ : PhantomData < Ty >) -> Self { if Ty :: is_directed () { EdgeProperty :: Directed } else { EdgeProperty :: Undirected } } }
};
}
