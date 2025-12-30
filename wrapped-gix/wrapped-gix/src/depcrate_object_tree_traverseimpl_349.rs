// Generated macro for impl_349 (impl)
macro_rules! Depcrate_object_tree_traverseimpl_349 {
() => {
// Module: crate::object::tree::traverse
// Provides: {"impl_349"}
// Dependencies: {}
impl BreadthFirstPresets < '_ , '_ > { # [doc = " Returns all entries and their file paths, recursively, as reachable from this tree."] pub fn files (& self) -> Result < Vec < gix_traverse :: tree :: recorder :: Entry > , gix_traverse :: tree :: breadthfirst :: Error > { let mut recorder = gix_traverse :: tree :: Recorder :: default () ; Platform { root : self . root , breadthfirst : * self , } . breadthfirst (& mut recorder) ? ; Ok (recorder . records) } }
};
}
