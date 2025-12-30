// Generated macro for print_crate_as_interface (function)
macro_rules! Depcrate_pprust_stateprint_crate_as_interface {
() => {
// Module: crate::pprust::state
// Provides: {"print_crate_as_interface"}
// Dependencies: {}
pub fn print_crate_as_interface (krate : & ast :: Crate , edition : Edition , g : & AttrIdGenerator ,) -> String { let mut s = State { s : pp :: Printer :: new () , comments : None , ann : & NoAnn , is_sdylib_interface : true } ; print_crate_inner (& mut s , krate , false , edition , g) ; s . s . eof () }
};
}
