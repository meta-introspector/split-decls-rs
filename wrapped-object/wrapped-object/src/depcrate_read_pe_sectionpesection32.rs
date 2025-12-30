// Generated macro for PeSection32 (type)
macro_rules! Depcrate_read_pe_sectionPeSection32 {
() => {
// Module: crate::read::pe::section
// Provides: {"PeSection32"}
// Dependencies: {}
# [doc = " A section in a [`PeFile32`](super::PeFile32)."] pub type PeSection32 < 'data , 'file , R = & 'data [u8] > = PeSection < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
};
}
