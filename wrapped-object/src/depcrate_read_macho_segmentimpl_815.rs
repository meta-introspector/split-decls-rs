// Generated macro for impl_815 (impl)
macro_rules! Depcrate_read_macho_segmentimpl_815 {
() => {
// Module: crate::read::macho::segment
// Provides: {"impl_815"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > Iterator for MachOSegmentIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOSegment < 'data , 'file , Mach , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| internal | MachOSegment { file : self . file , internal , }) } }
};
}
