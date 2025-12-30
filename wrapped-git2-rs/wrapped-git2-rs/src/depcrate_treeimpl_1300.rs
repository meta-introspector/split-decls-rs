// Generated macro for impl_1300 (impl)
macro_rules! Depcrate_treeimpl_1300 {
() => {
// Module: crate::tree
// Provides: {"impl_1300"}
// Dependencies: {}
impl < 'tree > DoubleEndedIterator for TreeIter < 'tree > { fn next_back (& mut self) -> Option < TreeEntry < 'tree > > { self . range . next_back () . and_then (| i | self . tree . get (i)) } }
};
}
