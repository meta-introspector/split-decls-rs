// Generated macro for impl_285 (impl)
macro_rules! Depcrate_read_coff_sectionimpl_285 {
() => {
// Module: crate::read::coff::section
// Provides: {"impl_285"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffSegmentIterator < 'data , 'file , R , Coff > { type Item = CoffSegment < 'data , 'file , R , Coff > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| section | CoffSegment { file : self . file , section , }) } }
};
}
