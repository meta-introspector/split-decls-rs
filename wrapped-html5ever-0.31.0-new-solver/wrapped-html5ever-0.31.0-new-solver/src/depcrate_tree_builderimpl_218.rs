// Generated macro for impl_218 (impl)
macro_rules! Depcrate_tree_builderimpl_218 {
() => {
// Module: crate::tree_builder
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'a , Handle > Iterator for ActiveFormattingIter < 'a , Handle > { type Item = (usize , & 'a Handle , & 'a Tag) ; fn next (& mut self) -> Option < (usize , & 'a Handle , & 'a Tag) > { match self . iter . next () { None | Some ((_ , & Marker)) => None , Some ((i , Element (h , t))) => Some ((i , h , t)) , } } }
};
}
