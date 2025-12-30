// Generated macro for impl_844 (impl)
macro_rules! Depcrate_strategy_fuseimpl_844 {
() => {
// Module: crate::strategy::fuse
// Provides: {"impl_844"}
// Dependencies: {}
impl < T : Strategy > Strategy for Fuse < T > { type Tree = Fuse < T :: Tree > ; type Value = T :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . inner . new_tree (runner) . map (Fuse :: new) } }
};
}
