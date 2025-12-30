// Generated macro for print_macho64 (function)
macro_rules! Depcrate_readobj_machoprint_macho64 {
() => {
// Module: crate::readobj::macho
// Provides: {"print_macho64"}
// Dependencies: {}
pub (super) fn print_macho64 (p : & mut Printer < '_ > , data : & [u8] , offset : u64 , cache : Option < & DyldCache > ,) { if let Some (header) = MachHeader64 :: parse (data , offset) . print_err (p) { writeln ! (p . w () , "Format: Mach-O 64-bit") . unwrap () ; print_macho (p , header , data , offset , cache) ; } }
};
}
