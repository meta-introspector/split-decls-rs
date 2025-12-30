// Generated macro for impl_929 (impl)
macro_rules! Depcrate_read_pe_fileimpl_929 {
() => {
// Module: crate::read::pe::file
// Provides: {"impl_929"}
// Dependencies: {}
impl < 'data , 'file , Pe , R > Iterator for PeComdatSectionIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { None } }
};
}
