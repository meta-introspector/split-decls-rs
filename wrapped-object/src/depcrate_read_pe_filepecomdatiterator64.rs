// Generated macro for PeComdatIterator64 (type)
macro_rules! Depcrate_read_pe_filePeComdatIterator64 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdatIterator64"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`PeFile64`]."] pub type PeComdatIterator64 < 'data , 'file , R = & 'data [u8] > = PeComdatIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
};
}
