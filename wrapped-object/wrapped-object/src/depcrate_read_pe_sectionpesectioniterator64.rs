// Generated macro for PeSectionIterator64 (type)
macro_rules! Depcrate_read_pe_sectionPeSectionIterator64 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`PeFile64`](super::PeFile64)."] pub type PeSectionIterator64 < 'data , 'file , R = & 'data [u8] > = PeSectionIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
};
}
