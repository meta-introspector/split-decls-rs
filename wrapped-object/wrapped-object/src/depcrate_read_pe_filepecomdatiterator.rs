// Generated macro for PeComdatIterator (struct)
macro_rules! Depcrate_read_pe_filePeComdatIterator {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdatIterator"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`PeFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct PeComdatIterator < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file PeFile < 'data , Pe , R > , }
};
}
