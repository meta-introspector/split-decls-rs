// Generated macro for print_coff_big (function)
macro_rules! Depcrate_readobj_peprint_coff_big {
() => {
// Module: crate::readobj::pe
// Provides: {"print_coff_big"}
// Dependencies: {}
pub (super) fn print_coff_big (p : & mut Printer < '_ > , data : & [u8]) { let mut offset = 0 ; if let Some (header) = AnonObjectHeaderBigobj :: parse (data , & mut offset) . print_err (p) { writeln ! (p . w () , "Format: COFF bigobj") . unwrap () ; print_bigobj (p , header) ; let sections = header . sections (data , offset) . print_err (p) ; let symbols = header . symbols (data) . print_err (p) ; if let Some (ref sections) = sections { print_sections (p , data , header . machine . get (LE) , symbols . as_ref () , sections) ; } if let Some (ref symbols) = symbols { print_symbols (p , sections . as_ref () , symbols) ; } } }
};
}
