// Generated macro for impl_198 (impl)
macro_rules! Depcrate_read_anyimpl_198 {
() => {
// Module: crate::read::any
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > Iterator for ComdatIterator < 'data , 'file , R > { type Item = Comdat < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { next_inner ! (self . inner , ComdatIteratorInternal , ComdatInternal) . map (| inner | Comdat { inner }) } }
};
}
