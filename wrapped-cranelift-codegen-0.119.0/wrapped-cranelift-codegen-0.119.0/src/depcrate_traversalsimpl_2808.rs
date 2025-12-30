// Generated macro for impl_2808 (impl)
macro_rules! Depcrate_traversalsimpl_2808 {
() => {
// Module: crate::traversals
// Provides: {"impl_2808"}
// Dependencies: {}
impl Iterator for DfsPostOrderIter < '_ > { type Item = ir :: Block ; fn next (& mut self) -> Option < Self :: Item > { loop { match self . 0 . next () ? { (Event :: Exit , b) => return Some (b) , (Event :: Enter , _) => continue , } } } }
};
}
