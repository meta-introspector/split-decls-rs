// Generated macro for Externals (struct)
macro_rules! Depcrate_graph_impl_stable_graphExternals {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"Externals"}
// Dependencies: {}
# [doc = " An iterator over either the nodes without edges to them or from them."] # [derive (Debug , Clone)] pub struct Externals < 'a , N : 'a , Ty , Ix : IndexType = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Node < Option < N > , Ix > > > , dir : Direction , ty : PhantomData < Ty > , }
};
}
