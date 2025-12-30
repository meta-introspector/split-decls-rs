// Generated macro for XcoffComdat (struct)
macro_rules! Depcrate_read_xcoff_comdatXcoffComdat {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"XcoffComdat"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffComdat < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file XcoffFile < 'data , Xcoff , R > , }
};
}
