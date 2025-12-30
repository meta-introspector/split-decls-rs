// Generated macro for impl_299 (impl)
macro_rules! Depcrate_acyclicimpl_299 {
() => {
// Module: crate::acyclic
// Provides: {"impl_299"}
// Dependencies: {}
impl < G : Visitable > Acyclic < G > { # [doc = " Create a new empty acyclic graph."] pub fn new () -> Self where G : Default , { Default :: default () } # [doc = " Get an iterator over the nodes, ordered by their position."] pub fn nodes_iter (& self) -> impl Iterator < Item = G :: NodeId > + '_ { self . order_map . nodes_iter () } # [doc = " Get an iterator over the nodes within the range of positions."] # [doc = ""] # [doc = " The nodes are ordered by their position in the topological sort."] pub fn range < 'r > (& 'r self , range : impl RangeBounds < TopologicalPosition > + 'r ,) -> impl Iterator < Item = G :: NodeId > + 'r { self . order_map . range (range) } # [doc = " Get the underlying graph."] pub fn inner (& self) -> & G { & self . graph } # [doc = " Get the underlying graph mutably."] # [doc = ""] # [doc = " This cannot be public because it might break the acyclicity invariant."] fn inner_mut (& mut self) -> & mut G { & mut self . graph } # [doc = " Consume the `Acyclic` wrapper and return the underlying graph."] pub fn into_inner (self) -> G { self . graph } }
};
}
