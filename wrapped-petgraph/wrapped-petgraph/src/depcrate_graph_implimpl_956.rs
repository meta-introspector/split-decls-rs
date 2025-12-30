// Generated macro for impl_956 (impl)
macro_rules! Depcrate_graph_implimpl_956 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_956"}
// Dependencies: {}
impl < N , E , Ty , Ix > Graph < N , E , Ty , Ix > { # [doc = " Create a new `Graph` with estimated capacity."] pub fn with_capacity (nodes : usize , edges : usize) -> Self { Graph { nodes : Vec :: with_capacity (nodes) , edges : Vec :: with_capacity (edges) , ty : PhantomData , } } }
};
}
