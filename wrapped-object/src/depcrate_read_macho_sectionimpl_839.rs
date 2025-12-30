// Generated macro for impl_839 (impl)
macro_rules! Depcrate_read_macho_sectionimpl_839 {
() => {
// Module: crate::read::macho::section
// Provides: {"impl_839"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > Iterator for MachOSectionIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOSection < 'data , 'file , Mach , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| & internal | MachOSection { file : self . file , internal , }) } }
};
}
