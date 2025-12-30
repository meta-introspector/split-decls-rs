// Generated macro for MachOSectionIterator (struct)
macro_rules! Depcrate_read_macho_sectionMachOSectionIterator {
() => {
// Module: crate::read::macho::section
// Provides: {"MachOSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`MachOFile`]."] pub struct MachOSectionIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , pub (super) iter : slice :: Iter < 'file , MachOSectionInternal < 'data , Mach , R > > , }
};
}
