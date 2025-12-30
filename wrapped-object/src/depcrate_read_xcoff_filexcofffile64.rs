// Generated macro for XcoffFile64 (type)
macro_rules! Depcrate_read_xcoff_fileXcoffFile64 {
() => {
// Module: crate::read::xcoff::file
// Provides: {"XcoffFile64"}
// Dependencies: {}
# [doc = " A 64-bit XCOFF object file."] # [doc = ""] # [doc = " This is a file that starts with [`xcoff::FileHeader64`], and corresponds"] # [doc = " to [`crate::FileKind::Xcoff64`]."] pub type XcoffFile64 < 'data , R = & 'data [u8] > = XcoffFile < 'data , xcoff :: FileHeader64 , R > ;
};
}
