// Generated macro for print_pe32 (function)
macro_rules! Depcrate_readobj_peprint_pe32 {
() => {
// Module: crate::readobj::pe
// Provides: {"print_pe32"}
// Dependencies: {}
pub (super) fn print_pe32 (p : & mut Printer < '_ > , data : & [u8]) { writeln ! (p . w () , "Format: PE 32-bit") . unwrap () ; print_pe :: < ImageNtHeaders32 > (p , data) ; }
};
}
