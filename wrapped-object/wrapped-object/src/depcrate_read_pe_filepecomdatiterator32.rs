// Generated macro for PeComdatIterator32 (type)
macro_rules! Depcrate_read_pe_filePeComdatIterator32 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdatIterator32"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`PeFile32`]."] pub type PeComdatIterator32 < 'data , 'file , R = & 'data [u8] > = PeComdatIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
};
}
