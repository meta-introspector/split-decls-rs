// Generated macro for impl_960 (impl)
macro_rules! Depcrate_read_pe_sectionimpl_960 {
() => {
// Module: crate::read::pe::section
// Provides: {"impl_960"}
// Dependencies: {}
impl < 'data , 'file , Pe , R > Iterator for PeSectionIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = PeSection < 'data , 'file , Pe , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | PeSection { file : self . file , index : SectionIndex (index + 1) , section , }) } }
};
}
