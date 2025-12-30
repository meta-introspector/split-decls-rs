// Generated macro for XcoffComdatIterator (struct)
macro_rules! Depcrate_read_xcoff_comdatXcoffComdatIterator {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"XcoffComdatIterator"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffComdatIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] pub (crate) file : & 'file XcoffFile < 'data , Xcoff , R > , }
};
}
