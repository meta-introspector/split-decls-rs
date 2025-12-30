// Generated macro for impl_920 (impl)
macro_rules! Depcrate_read_pe_fileimpl_920 {
() => {
// Module: crate::read::pe::file
// Provides: {"impl_920"}
// Dependencies: {}
impl < 'data , 'file , Pe , R > Iterator for PeComdatIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = PeComdat < 'data , 'file , Pe , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
};
}
