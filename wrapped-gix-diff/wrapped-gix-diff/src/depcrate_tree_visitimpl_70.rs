// Generated macro for impl_70 (impl)
macro_rules! Depcrate_tree_visitimpl_70 {
() => {
// Module: crate::tree::visit
// Provides: {"impl_70"}
// Dependencies: {}
impl Action { # [doc = " Returns true if this action means to stop the traversal."] pub fn cancelled (& self) -> bool { matches ! (self , Action :: Cancel) } }
};
}
