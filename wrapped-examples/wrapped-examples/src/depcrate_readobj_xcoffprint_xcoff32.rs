// Generated macro for print_xcoff32 (function)
macro_rules! Depcrate_readobj_xcoffprint_xcoff32 {
() => {
// Module: crate::readobj::xcoff
// Provides: {"print_xcoff32"}
// Dependencies: {}
pub (super) fn print_xcoff32 (p : & mut Printer < '_ > , data : & [u8]) { let mut offset = 0 ; if let Some (header) = FileHeader32 :: parse (data , & mut offset) . print_err (p) { writeln ! (p . w () , "Format: XCOFF 32-bit") . unwrap () ; print_xcoff (p , header , data , offset) ; } }
};
}
