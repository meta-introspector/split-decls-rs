// Generated macro for impl_346 (impl)
macro_rules! Depcrate_object_tree_traverseimpl_346 {
() => {
// Module: crate::object::tree::traverse
// Provides: {"impl_346"}
// Dependencies: {}
# [doc = " Traversal"] impl < 'repo > Tree < 'repo > { # [doc = " Obtain a platform for initiating a variety of traversals."] pub fn traverse (& self) -> Platform < '_ , 'repo > { Platform { root : self , breadthfirst : BreadthFirstPresets { root : self } , } } }
};
}
