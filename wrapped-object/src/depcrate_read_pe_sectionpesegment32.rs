// Generated macro for PeSegment32 (type)
macro_rules! Depcrate_read_pe_sectionPeSegment32 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSegment32"}
// Dependencies: {}
# [doc = " A loadable section in a [`PeFile32`](super::PeFile32)."] pub type PeSegment32 < 'data , 'file , R = & 'data [u8] > = PeSegment < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
};
}
