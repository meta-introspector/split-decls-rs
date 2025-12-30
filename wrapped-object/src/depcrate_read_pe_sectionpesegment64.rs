// Generated macro for PeSegment64 (type)
macro_rules! Depcrate_read_pe_sectionPeSegment64 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSegment64"}
// Dependencies: {}
# [doc = " A loadable section in a [`PeFile64`](super::PeFile64)."] pub type PeSegment64 < 'data , 'file , R = & 'data [u8] > = PeSegment < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
};
}
