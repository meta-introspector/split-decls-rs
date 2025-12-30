// Generated macro for impl_765 (impl)
macro_rules! Depcrate_read_macho_fileimpl_765 {
() => {
// Module: crate::read::macho::file
// Provides: {"impl_765"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > Iterator for MachOComdatIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOComdat < 'data , 'file , Mach , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
};
}
