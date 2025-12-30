// Generated macro for impl_20 (impl)
macro_rules! Depcrate_rootimpl_20 {
() => {
// Module: crate::root
// Provides: {"impl_20"}
// Dependencies: {}
impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let inner = self . inner . next () ? ; Some (inner . map (| inner | DirEntry { inner })) } }
};
}
