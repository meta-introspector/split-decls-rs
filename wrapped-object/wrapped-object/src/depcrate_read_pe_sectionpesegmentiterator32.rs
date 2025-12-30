// Generated macro for PeSegmentIterator32 (type)
macro_rules! Depcrate_read_pe_sectionPeSegmentIterator32 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSegmentIterator32"}
// Dependencies: {}
# [doc = " An iterator for the loadable sections in a [`PeFile32`](super::PeFile32)."] pub type PeSegmentIterator32 < 'data , 'file , R = & 'data [u8] > = PeSegmentIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
};
}
