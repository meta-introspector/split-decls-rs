// Generated macro for impl_15 (impl)
macro_rules! Depcrate_tree_itemimpl_15 {
() => {
// Module: crate::tree::item
// Provides: {"impl_15"}
// Dependencies: {}
impl crate :: Count for Item { fn set (& self , step : usize) { Item :: set (self , step) } fn step (& self) -> usize { Item :: step (self) . unwrap_or (0) } fn inc_by (& self , step : usize) { self . inc_by (step) } fn counter (& self) -> StepShared { Arc :: clone (& self . value) } }
};
}
