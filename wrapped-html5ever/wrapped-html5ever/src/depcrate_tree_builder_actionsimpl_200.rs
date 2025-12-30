// Generated macro for impl_200 (impl)
macro_rules! Depcrate_tree_builder_actionsimpl_200 {
() => {
// Module: crate::tree_builder::actions
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'a , Handle > Iterator for ActiveFormattingIter < 'a , Handle > { type Item = (usize , & 'a Handle , & 'a Tag) ; fn next (& mut self) -> Option < (usize , & 'a Handle , & 'a Tag) > { match self . iter . next () { None | Some ((_ , & Marker)) => None , Some ((i , & Element (ref h , ref t))) => Some ((i , h , t)) , } } }
};
}
