// Generated macro for impl_77 (impl)
macro_rules! Depcrate_nodes_hamtimpl_77 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_77"}
// Dependencies: {}
impl < A > Entry < A > { fn is_value (& self) -> bool { matches ! (self , Entry :: Value (_ , _)) } fn unwrap_value (self) -> A { match self { Entry :: Value (a , _) => a , _ => panic ! ("nodes::hamt::Entry::unwrap_value: unwrapped a non-value") , } } fn from_node (pool : & Pool < Node < A > > , node : Node < A >) -> Self { Entry :: Node (PoolRef :: new (pool , node)) } }
};
}
