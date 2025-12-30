// Generated macro for PeComdatSectionIterator (struct)
macro_rules! Depcrate_read_pe_filePeComdatSectionIterator {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdatSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`PeFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct PeComdatSectionIterator < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file PeFile < 'data , Pe , R > , }
};
}
