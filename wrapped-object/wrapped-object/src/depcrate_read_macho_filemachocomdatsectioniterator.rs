// Generated macro for MachOComdatSectionIterator (struct)
macro_rules! Depcrate_read_macho_fileMachOComdatSectionIterator {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOComdatSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`MachOFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct MachOComdatSectionIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file MachOFile < 'data , Mach , R > , }
};
}
