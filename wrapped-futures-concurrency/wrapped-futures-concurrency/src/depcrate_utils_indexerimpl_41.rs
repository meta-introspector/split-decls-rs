// Generated macro for impl_41 (impl)
macro_rules! Depcrate_utils_indexerimpl_41 {
() => {
// Module: crate::utils::indexer
// Provides: {"impl_41"}
// Dependencies: {}
impl Iterator for IndexIter { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| pos | (pos + self . offset) . wrapping_rem (self . iter . end)) } }
};
}
