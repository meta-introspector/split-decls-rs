// Generated macro for impl_954 (impl)
macro_rules! Depcrate_graph_implimpl_954 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_954"}
// Dependencies: {}
impl < N , E > Graph < N , E , Directed > { # [doc = " Create a new `Graph` with directed edges."] # [doc = ""] # [doc = " This is a convenience method. Use `Graph::with_capacity` or `Graph::default` for"] # [doc = " a constructor that is generic in all the type parameters of `Graph`."] pub fn new () -> Self { Graph { nodes : Vec :: new () , edges : Vec :: new () , ty : PhantomData , } } }
};
}
