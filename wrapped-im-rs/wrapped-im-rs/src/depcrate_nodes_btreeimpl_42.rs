// Generated macro for impl_42 (impl)
macro_rules! Depcrate_nodes_btreeimpl_42 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_42"}
// Dependencies: {}
impl < A : Clone > ConsumingIter < A > { pub (crate) fn new (root : & Node < A > , total : usize) -> Self { ConsumingIter { fwd_last : None , fwd_stack : vec ! [ConsumingIterItem :: Consider (root . clone ())] , back_last : None , back_stack : vec ! [ConsumingIterItem :: Consider (root . clone ())] , remaining : total , } } fn push_node (stack : & mut Vec < ConsumingIterItem < A > > , maybe_node : Option < PoolRef < Node < A > > >) { if let Some (node) = maybe_node { stack . push (ConsumingIterItem :: Consider (PoolRef :: unwrap_or_clone (node))) } } fn push (stack : & mut Vec < ConsumingIterItem < A > > , mut node : Node < A >) { for _n in 0 .. node . keys . len () { ConsumingIter :: push_node (stack , node . children . pop_back ()) ; stack . push (ConsumingIterItem :: Yield (node . keys . pop_back ())) ; } ConsumingIter :: push_node (stack , node . children . pop_back ()) ; } fn push_fwd (& mut self , node : Node < A >) { ConsumingIter :: push (& mut self . fwd_stack , node) } fn push_node_back (& mut self , maybe_node : Option < PoolRef < Node < A > > >) { if let Some (node) = maybe_node { self . back_stack . push (ConsumingIterItem :: Consider (PoolRef :: unwrap_or_clone (node))) } } fn push_back (& mut self , mut node : Node < A >) { for _i in 0 .. node . keys . len () { self . push_node_back (node . children . pop_front ()) ; self . back_stack . push (ConsumingIterItem :: Yield (node . keys . pop_front ())) ; } self . push_node_back (node . children . pop_back ()) ; } }
};
}
