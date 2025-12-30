// Generated macro for Externals (struct)
macro_rules! Depcrate_graph_implExternals {
() => {
// Module: crate::graph_impl
// Provides: {"Externals"}
// Dependencies: {}
# [doc = " An iterator over either the nodes without edges to them or from them."] # [derive (Debug , Clone)] pub struct Externals < 'a , N : 'a , Ty , Ix : IndexType = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Node < N , Ix > > > , dir : Direction , ty : PhantomData < Ty > , }
};
}
