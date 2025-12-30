// Generated macro for print_object (function)
macro_rules! Depcrate_readobjprint_object {
() => {
// Module: crate::readobj
// Provides: {"print_object"}
// Dependencies: {}
fn print_object (p : & mut Printer < '_ > , data : & [u8] , extra_files : & [& [u8]]) { let kind = match object :: FileKind :: parse (data) { Ok (file) => file , Err (err) => { println ! ("Failed to parse file: {}" , err) ; return ; } } ; match kind { object :: FileKind :: Archive => print_archive (p , data) , object :: FileKind :: Coff => pe :: print_coff (p , data) , object :: FileKind :: CoffBig => pe :: print_coff_big (p , data) , object :: FileKind :: CoffImport => pe :: print_coff_import (p , data) , object :: FileKind :: DyldCache => macho :: print_dyld_cache (p , data , extra_files) , object :: FileKind :: Elf32 => elf :: print_elf32 (p , data) , object :: FileKind :: Elf64 => elf :: print_elf64 (p , data) , object :: FileKind :: MachO32 => macho :: print_macho32 (p , data , 0 , None) , object :: FileKind :: MachO64 => macho :: print_macho64 (p , data , 0 , None) , object :: FileKind :: MachOFat32 => macho :: print_macho_fat32 (p , data) , object :: FileKind :: MachOFat64 => macho :: print_macho_fat64 (p , data) , object :: FileKind :: Pe32 => pe :: print_pe32 (p , data) , object :: FileKind :: Pe64 => pe :: print_pe64 (p , data) , object :: FileKind :: Xcoff32 => xcoff :: print_xcoff32 (p , data) , object :: FileKind :: Xcoff64 => xcoff :: print_xcoff64 (p , data) , _ => { } } }
};
}
