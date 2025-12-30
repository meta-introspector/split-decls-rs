// Generated macro for print_file_header (function)
macro_rules! Depcrate_readobj_xcoffprint_file_header {
() => {
// Module: crate::readobj::xcoff
// Provides: {"print_file_header"}
// Dependencies: {}
fn print_file_header < Xcoff : FileHeader > (p : & mut Printer < '_ > , header : & Xcoff) { if ! p . options . file { return ; } p . group ("FileHeader" , | p | { p . field_hex ("Magic" , header . f_magic ()) ; p . field ("NumberOfSections" , header . f_nscns ()) ; p . field_hex ("TimeDate" , header . f_timdat ()) ; p . field_hex ("SymbolPointer" , header . f_symptr () . into ()) ; p . field ("NumberOfSymbols" , header . f_nsyms ()) ; p . field_hex ("SizeOfOptionalHeader" , header . f_opthdr ()) ; p . field_hex ("Flags" , header . f_flags ()) ; p . flags (header . f_flags () , 0 , FLAGS_F) ; }) ; }
};
}
