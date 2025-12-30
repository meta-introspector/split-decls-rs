// Generated macro for PeSegmentIterator64 (type)
macro_rules! Depcrate_read_pe_sectionPeSegmentIterator64 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSegmentIterator64"}
// Dependencies: {}
# [doc = " An iterator for the loadable sections in a [`PeFile64`](super::PeFile64)."] pub type PeSegmentIterator64 < 'data , 'file , R = & 'data [u8] > = PeSegmentIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
};
}
