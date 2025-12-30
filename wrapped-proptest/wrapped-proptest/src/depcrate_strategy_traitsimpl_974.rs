// Generated macro for impl_974 (impl)
macro_rules! Depcrate_strategy_traitsimpl_974 {
() => {
// Module: crate::strategy::traits
// Provides: {"impl_974"}
// Dependencies: {}
impl < T : Strategy > Strategy for NoShrink < T > { type Tree = NoShrink < T :: Tree > ; type Value = T :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . 0 . new_tree (runner) . map (NoShrink) } }
};
}
