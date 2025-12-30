// Generated macro for impl_838 (impl)
macro_rules! Depcrate_read_macho_sectionimpl_838 {
() => {
// Module: crate::read::macho::section
// Provides: {"impl_838"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > fmt :: Debug for MachOSectionIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MachOSectionIterator") . finish () } }
};
}
