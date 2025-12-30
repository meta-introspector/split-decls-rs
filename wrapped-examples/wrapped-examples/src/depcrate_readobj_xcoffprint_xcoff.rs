// Generated macro for print_xcoff (function)
macro_rules! Depcrate_readobj_xcoffprint_xcoff {
() => {
// Module: crate::readobj::xcoff
// Provides: {"print_xcoff"}
// Dependencies: {}
fn print_xcoff < Xcoff : FileHeader > (p : & mut Printer < '_ > , header : & Xcoff , data : & [u8] , mut offset : u64 ,) { print_file_header (p , header) ; if let Some (aux_header) = header . aux_header (data , & mut offset) . print_err (p) { let sections = header . sections (data , & mut offset) . print_err (p) ; let symbols = header . symbols (data) . print_err (p) ; if let Some (aux_header) = aux_header { print_aux_header (p , aux_header) ; } if let Some (ref sections) = sections { print_sections (p , data , symbols . as_ref () , sections) ; } if let Some (ref symbols) = symbols { print_symbols (p , sections . as_ref () , symbols) ; } } }
};
}
