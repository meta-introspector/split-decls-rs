// Generated macro for MachOFatFile (struct)
macro_rules! Depcrate_read_macho_fatMachOFatFile {
() => {
// Module: crate::read::macho::fat
// Provides: {"MachOFatFile"}
// Dependencies: {}
# [doc = " A Mach-O universal binary."] # [doc = ""] # [doc = " This is a file that starts with [`macho::FatHeader`], and corresponds"] # [doc = " to [`crate::FileKind::MachOFat32`] or [`crate::FileKind::MachOFat64`]."] # [derive (Debug , Clone)] pub struct MachOFatFile < 'data , Fat : FatArch > { header : & 'data macho :: FatHeader , arches : & 'data [Fat] , }
};
}
