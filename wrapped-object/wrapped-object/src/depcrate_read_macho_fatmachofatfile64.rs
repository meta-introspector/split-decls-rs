// Generated macro for MachOFatFile64 (type)
macro_rules! Depcrate_read_macho_fatMachOFatFile64 {
() => {
// Module: crate::read::macho::fat
// Provides: {"MachOFatFile64"}
// Dependencies: {}
# [doc = " A 64-bit Mach-O universal binary."] # [doc = ""] # [doc = " This is a file that starts with [`macho::FatHeader`], and corresponds"] # [doc = " to [`crate::FileKind::MachOFat64`]."] pub type MachOFatFile64 < 'data > = MachOFatFile < 'data , macho :: FatArch64 > ;
};
}
