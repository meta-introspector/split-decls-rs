// Generated macro for impl_190 (impl)
macro_rules! Depcrate_read_anyimpl_190 {
() => {
// Module: crate::read::any
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SectionIterator < 'data , 'file , R > { type Item = Section < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { next_inner ! (self . inner , SectionIteratorInternal , SectionInternal) . map (| inner | Section { inner }) } }
};
}
