// Generated macro for print_pe64 (function)
macro_rules! Depcrate_readobj_peprint_pe64 {
() => {
// Module: crate::readobj::pe
// Provides: {"print_pe64"}
// Dependencies: {}
pub (super) fn print_pe64 (p : & mut Printer < '_ > , data : & [u8]) { writeln ! (p . w () , "Format: PE 64-bit") . unwrap () ; print_pe :: < ImageNtHeaders64 > (p , data) ; }
};
}
