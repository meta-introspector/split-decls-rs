// Generated macro for impl_70 (impl)
macro_rules! Depcrate_visit_traversalimpl_70 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_70"}
// Dependencies: {}
impl < W , C > WalkerIter < W , C > where W : Walker < C > , C : Clone , { pub fn context (& self) -> C { self . context . clone () } pub fn inner_ref (& self) -> & W { & self . walker } pub fn inner_mut (& mut self) -> & mut W { & mut self . walker } }
};
}
