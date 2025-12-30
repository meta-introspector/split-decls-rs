// Generated macro for impl_1145 (impl)
macro_rules! Depcrate_read_xcoff_sectionimpl_1145 {
() => {
// Module: crate::read::xcoff::section
// Provides: {"impl_1145"}
// Dependencies: {}
impl < 'data , 'file , Xcoff , R > Iterator for XcoffSectionIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = XcoffSection < 'data , 'file , Xcoff , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | XcoffSection { index : SectionIndex (index + 1) , file : self . file , section , }) } }
};
}
