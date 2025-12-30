// Generated macro for impl_1236 (impl)
macro_rules! Depcrate_read_xcoff_comdatimpl_1236 {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"impl_1236"}
// Dependencies: {}
impl < 'data , 'file , Xcoff , R > Iterator for XcoffComdatSectionIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { None } }
};
}
