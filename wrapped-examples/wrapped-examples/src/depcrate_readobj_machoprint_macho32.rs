// Generated macro for print_macho32 (function)
macro_rules! Depcrate_readobj_machoprint_macho32 {
() => {
// Module: crate::readobj::macho
// Provides: {"print_macho32"}
// Dependencies: {}
pub (super) fn print_macho32 (p : & mut Printer < '_ > , data : & [u8] , offset : u64 , cache : Option < & DyldCache > ,) { if let Some (header) = MachHeader32 :: parse (data , offset) . print_err (p) { writeln ! (p . w () , "Format: Mach-O 32-bit") . unwrap () ; print_macho (p , header , data , offset , cache) ; } }
};
}
