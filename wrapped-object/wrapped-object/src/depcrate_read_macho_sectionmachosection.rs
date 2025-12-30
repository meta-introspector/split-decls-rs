// Generated macro for MachOSection (struct)
macro_rules! Depcrate_read_macho_sectionMachOSection {
() => {
// Module: crate::read::macho::section
// Provides: {"MachOSection"}
// Dependencies: {}
# [doc = " A section in a [`MachOFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct MachOSection < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , pub (super) internal : MachOSectionInternal < 'data , Mach , R > , }
};
}
