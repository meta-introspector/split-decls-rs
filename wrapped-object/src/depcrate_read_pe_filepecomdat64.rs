// Generated macro for PeComdat64 (type)
macro_rules! Depcrate_read_pe_filePeComdat64 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdat64"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`PeFile64`]."] pub type PeComdat64 < 'data , 'file , R = & 'data [u8] > = PeComdat < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
};
}
