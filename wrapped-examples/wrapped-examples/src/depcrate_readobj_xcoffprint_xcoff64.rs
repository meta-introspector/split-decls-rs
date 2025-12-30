// Generated macro for print_xcoff64 (function)
macro_rules! Depcrate_readobj_xcoffprint_xcoff64 {
() => {
// Module: crate::readobj::xcoff
// Provides: {"print_xcoff64"}
// Dependencies: {}
pub (super) fn print_xcoff64 (p : & mut Printer < '_ > , data : & [u8]) { let mut offset = 0 ; if let Some (header) = FileHeader64 :: parse (data , & mut offset) . print_err (p) { writeln ! (p . w () , "Format: XCOFF 64-bit") . unwrap () ; print_xcoff (p , header , data , offset) ; } }
};
}
