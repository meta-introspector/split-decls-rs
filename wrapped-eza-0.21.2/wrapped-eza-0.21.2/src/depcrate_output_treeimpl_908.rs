// Generated macro for impl_908 (impl)
macro_rules! Depcrate_output_treeimpl_908 {
() => {
// Module: crate::output::tree
// Provides: {"impl_908"}
// Dependencies: {}
impl < I , T > Iterator for Iter < I > where I : ExactSizeIterator + Iterator < Item = T > , { type Item = (TreeParams , T) ; fn next (& mut self) -> Option < Self :: Item > { let t = self . inner . next () ? ; let params = TreeParams :: new (self . current_depth , self . inner . len () == 0) ; Some ((params , t)) } }
};
}
