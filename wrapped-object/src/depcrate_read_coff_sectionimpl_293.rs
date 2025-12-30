// Generated macro for impl_293 (impl)
macro_rules! Depcrate_read_coff_sectionimpl_293 {
() => {
// Module: crate::read::coff::section
// Provides: {"impl_293"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffSectionIterator < 'data , 'file , R , Coff > { type Item = CoffSection < 'data , 'file , R , Coff > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | CoffSection { file : self . file , index : SectionIndex (index + 1) , section , }) } }
};
}
