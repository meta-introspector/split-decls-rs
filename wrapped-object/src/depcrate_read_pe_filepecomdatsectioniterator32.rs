// Generated macro for PeComdatSectionIterator32 (type)
macro_rules! Depcrate_read_pe_filePeComdatSectionIterator32 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdatSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`PeFile32`]."] pub type PeComdatSectionIterator32 < 'data , 'file , R = & 'data [u8] > = PeComdatSectionIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
};
}
