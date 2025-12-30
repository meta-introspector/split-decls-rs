// Generated macro for XcoffSection (struct)
macro_rules! Depcrate_read_xcoff_sectionXcoffSection {
() => {
// Module: crate::read::xcoff::section
// Provides: {"XcoffSection"}
// Dependencies: {}
# [doc = " A section in an [`XcoffFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct XcoffSection < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) section : & 'data Xcoff :: SectionHeader , pub (super) index : SectionIndex , }
};
}
