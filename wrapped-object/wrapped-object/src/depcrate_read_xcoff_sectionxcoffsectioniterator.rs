// Generated macro for XcoffSectionIterator (struct)
macro_rules! Depcrate_read_xcoff_sectionXcoffSectionIterator {
() => {
// Module: crate::read::xcoff::section
// Provides: {"XcoffSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in an [`XcoffFile`]."] # [derive (Debug)] pub struct XcoffSectionIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) iter : iter :: Enumerate < slice :: Iter < 'data , Xcoff :: SectionHeader > > , }
};
}
