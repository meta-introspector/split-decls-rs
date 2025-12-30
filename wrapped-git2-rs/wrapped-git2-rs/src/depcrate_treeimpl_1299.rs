// Generated macro for impl_1299 (impl)
macro_rules! Depcrate_treeimpl_1299 {
() => {
// Module: crate::tree
// Provides: {"impl_1299"}
// Dependencies: {}
impl < 'tree > Iterator for TreeIter < 'tree > { type Item = TreeEntry < 'tree > ; fn next (& mut self) -> Option < TreeEntry < 'tree > > { self . range . next () . and_then (| i | self . tree . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } fn nth (& mut self , n : usize) -> Option < TreeEntry < 'tree > > { self . range . nth (n) . and_then (| i | self . tree . get (i)) } }
};
}
