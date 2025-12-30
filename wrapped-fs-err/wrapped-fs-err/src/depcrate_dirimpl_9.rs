// Generated macro for impl_9 (impl)
macro_rules! Depcrate_dirimpl_9 {
() => {
// Module: crate::dir
// Provides: {"impl_9"}
// Dependencies: {}
impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < Self :: Item > { Some (self . inner . next () ? . map_err (| source | Error :: build (source , ErrorKind :: ReadDir , & self . path)) . map (| inner | DirEntry { inner }) ,) } }
};
}
