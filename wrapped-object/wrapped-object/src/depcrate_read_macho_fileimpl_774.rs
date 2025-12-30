// Generated macro for impl_774 (impl)
macro_rules! Depcrate_read_macho_fileimpl_774 {
() => {
// Module: crate::read::macho::file
// Provides: {"impl_774"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > Iterator for MachOComdatSectionIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { None } }
};
}
