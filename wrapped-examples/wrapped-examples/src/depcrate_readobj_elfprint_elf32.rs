// Generated macro for print_elf32 (function)
macro_rules! Depcrate_readobj_elfprint_elf32 {
() => {
// Module: crate::readobj::elf
// Provides: {"print_elf32"}
// Dependencies: {}
pub (super) fn print_elf32 (p : & mut Printer < '_ > , data : & [u8]) { if let Some (elf) = FileHeader32 :: < Endianness > :: parse (data) . print_err (p) { writeln ! (p . w () , "Format: ELF 32-bit") . unwrap () ; print_elf (p , elf , data) ; } }
};
}
