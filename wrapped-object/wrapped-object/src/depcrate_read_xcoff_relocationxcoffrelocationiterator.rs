// Generated macro for XcoffRelocationIterator (struct)
macro_rules! Depcrate_read_xcoff_relocationXcoffRelocationIterator {
() => {
// Module: crate::read::xcoff::relocation
// Provides: {"XcoffRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the relocations in an [`XcoffSection`](super::XcoffSection)."] pub struct XcoffRelocationIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) relocations : slice :: Iter < 'data , < < Xcoff as FileHeader > :: SectionHeader as SectionHeader > :: Rel > , }
};
}
