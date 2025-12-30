// Generated macro for impl_2806 (impl)
macro_rules! Depcrate_traversalsimpl_2806 {
() => {
// Module: crate::traversals
// Provides: {"impl_2806"}
// Dependencies: {}
impl Iterator for DfsPreOrderIter < '_ > { type Item = ir :: Block ; fn next (& mut self) -> Option < Self :: Item > { loop { match self . 0 . next () ? { (Event :: Enter , b) => return Some (b) , (Event :: Exit , _) => continue , } } } }
};
}
