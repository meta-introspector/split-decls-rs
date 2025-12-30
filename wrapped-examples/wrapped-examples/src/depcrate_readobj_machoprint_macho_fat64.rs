// Generated macro for print_macho_fat64 (function)
macro_rules! Depcrate_readobj_machoprint_macho_fat64 {
() => {
// Module: crate::readobj::macho
// Provides: {"print_macho_fat64"}
// Dependencies: {}
pub (super) fn print_macho_fat64 (p : & mut Printer < '_ > , data : & [u8]) { if let Some (fat) = MachOFatFile64 :: parse (data) . print_err (p) { writeln ! (p . w () , "Format: Mach-O Fat 64-bit") . unwrap () ; print_fat_header (p , fat . header ()) ; for arch in fat . arches () { print_fat_arch (p , arch) ; } for arch in fat . arches () { if let Some (data) = arch . data (data) . print_err (p) { p . blank () ; print_object (p , data , & []) ; } } } }
};
}
