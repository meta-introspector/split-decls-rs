// Generated macro for PeComdat32 (type)
macro_rules! Depcrate_read_pe_filePeComdat32 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdat32"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`PeFile32`]."] pub type PeComdat32 < 'data , 'file , R = & 'data [u8] > = PeComdat < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
};
}
