// Generated macro for impl_224 (impl)
macro_rules! Depcrate_read_anyimpl_224 {
() => {
// Module: crate::read::any
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SectionRelocationIterator < 'data , 'file , R > { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { with_inner_mut ! (self . inner , SectionRelocationIteratorInternal , | x | x . next ()) } }
};
}
