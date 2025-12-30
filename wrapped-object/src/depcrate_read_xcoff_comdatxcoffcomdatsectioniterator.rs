// Generated macro for XcoffComdatSectionIterator (struct)
macro_rules! Depcrate_read_xcoff_comdatXcoffComdatSectionIterator {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"XcoffComdatSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffComdatSectionIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file XcoffFile < 'data , Xcoff , R > , }
};
}
