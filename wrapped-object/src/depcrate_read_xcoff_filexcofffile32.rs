// Generated macro for XcoffFile32 (type)
macro_rules! Depcrate_read_xcoff_fileXcoffFile32 {
() => {
// Module: crate::read::xcoff::file
// Provides: {"XcoffFile32"}
// Dependencies: {}
# [doc = " A 32-bit XCOFF object file."] # [doc = ""] # [doc = " This is a file that starts with [`xcoff::FileHeader32`], and corresponds"] # [doc = " to [`crate::FileKind::Xcoff32`]."] pub type XcoffFile32 < 'data , R = & 'data [u8] > = XcoffFile < 'data , xcoff :: FileHeader32 , R > ;
};
}
