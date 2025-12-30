// Generated macro for impl_206 (impl)
macro_rules! Depcrate_read_anyimpl_206 {
() => {
// Module: crate::read::any
// Provides: {"impl_206"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > Iterator for ComdatSectionIterator < 'data , 'file , R > { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { with_inner_mut ! (self . inner , ComdatSectionIteratorInternal , | x | x . next ()) } }
};
}
