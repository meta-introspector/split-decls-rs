// Generated macro for print_elf64 (function)
macro_rules! Depcrate_readobj_elfprint_elf64 {
() => {
// Module: crate::readobj::elf
// Provides: {"print_elf64"}
// Dependencies: {}
pub (super) fn print_elf64 (p : & mut Printer < '_ > , data : & [u8]) { if let Some (elf) = FileHeader64 :: < Endianness > :: parse (data) . print_err (p) { writeln ! (p . w () , "Format: ELF 64-bit") . unwrap () ; print_elf (p , elf , data) ; } }
};
}
