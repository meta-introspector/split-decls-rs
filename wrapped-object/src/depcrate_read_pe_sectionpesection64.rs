// Generated macro for PeSection64 (type)
macro_rules! Depcrate_read_pe_sectionPeSection64 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSection64"}
// Dependencies: {}
# [doc = " A section in a [`PeFile64`](super::PeFile64)."] pub type PeSection64 < 'data , 'file , R = & 'data [u8] > = PeSection < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
};
}
