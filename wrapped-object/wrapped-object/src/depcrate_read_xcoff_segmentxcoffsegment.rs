// Generated macro for XcoffSegment (struct)
macro_rules! Depcrate_read_xcoff_segmentXcoffSegment {
() => {
// Module: crate::read::xcoff::segment
// Provides: {"XcoffSegment"}
// Dependencies: {}
# [doc = " A loadable section in an [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffSegment < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , }
};
}
