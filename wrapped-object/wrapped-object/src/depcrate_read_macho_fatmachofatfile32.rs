// Generated macro for MachOFatFile32 (type)
macro_rules! Depcrate_read_macho_fatMachOFatFile32 {
() => {
// Module: crate::read::macho::fat
// Provides: {"MachOFatFile32"}
// Dependencies: {}
# [doc = " A 32-bit Mach-O universal binary."] # [doc = ""] # [doc = " This is a file that starts with [`macho::FatHeader`], and corresponds"] # [doc = " to [`crate::FileKind::MachOFat32`]."] pub type MachOFatFile32 < 'data > = MachOFatFile < 'data , macho :: FatArch32 > ;
};
}
