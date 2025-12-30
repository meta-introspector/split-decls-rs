// Generated macro for impl_950 (impl)
macro_rules! Depcrate_read_pe_sectionimpl_950 {
() => {
// Module: crate::read::pe::section
// Provides: {"impl_950"}
// Dependencies: {}
impl < 'data , 'file , Pe , R > Iterator for PeSegmentIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = PeSegment < 'data , 'file , Pe , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| section | PeSegment { file : self . file , section , }) } }
};
}
