// Generated macro for impl_76 (impl)
macro_rules! Depcrate_nodes_hamtimpl_76 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_76"}
// Dependencies: {}
impl < A : Clone > Clone for Entry < A > { fn clone (& self) -> Self { match self { Entry :: Value (value , hash) => Entry :: Value (value . clone () , * hash) , Entry :: Collision (coll) => Entry :: Collision (coll . clone ()) , Entry :: Node (node) => Entry :: Node (node . clone ()) , } } }
};
}
