// Generated macro for PeComdatSectionIterator64 (type)
macro_rules! Depcrate_read_pe_filePeComdatSectionIterator64 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdatSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`PeFile64`]."] pub type PeComdatSectionIterator64 < 'data , 'file , R = & 'data [u8] > = PeComdatSectionIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
};
}
