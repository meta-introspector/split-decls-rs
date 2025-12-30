// Generated macro for PeSectionIterator32 (type)
macro_rules! Depcrate_read_pe_sectionPeSectionIterator32 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`PeFile32`](super::PeFile32)."] pub type PeSectionIterator32 < 'data , 'file , R = & 'data [u8] > = PeSectionIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
};
}
