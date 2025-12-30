// Generated macro for impl_304 (impl)
macro_rules! Depcrate_acyclicimpl_304 {
() => {
// Module: crate::acyclic
// Provides: {"impl_304"}
// Dependencies: {}
impl < G : Create + Visitable + NodeIndexable > Create for Acyclic < G > where for < 'a > & 'a G : IntoNeighborsDirected + IntoNodeIdentifiers + Visitable < Map = G :: Map > + GraphBase < NodeId = G :: NodeId > , G :: NodeId : IndexType , { fn with_capacity (nodes : usize , edges : usize) -> Self { let graph = G :: with_capacity (nodes , edges) ; let order_map = OrderMap :: with_capacity (nodes) ; let discovered = FixedBitSet :: with_capacity (nodes) ; let finished = FixedBitSet :: with_capacity (nodes) ; Self { graph , order_map , discovered : RefCell :: new (discovered) , finished : RefCell :: new (finished) , } } }
};
}
