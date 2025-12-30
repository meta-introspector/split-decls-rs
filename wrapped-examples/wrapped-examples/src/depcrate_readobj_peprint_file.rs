// Generated macro for print_file (function)
macro_rules! Depcrate_readobj_peprint_file {
() => {
// Module: crate::readobj::pe
// Provides: {"print_file"}
// Dependencies: {}
fn print_file (p : & mut Printer < '_ > , header : & ImageFileHeader) { if ! p . options . file { return ; } p . group ("ImageFileHeader" , | p | { p . field_enum ("Machine" , header . machine . get (LE) , FLAGS_IMAGE_FILE_MACHINE) ; p . field ("NumberOfSections" , header . number_of_sections . get (LE)) ; p . field ("TimeDateStamp" , header . time_date_stamp . get (LE)) ; p . field_hex ("PointerToSymbolTable" , header . pointer_to_symbol_table . get (LE) ,) ; p . field ("NumberOfSymbols" , header . number_of_symbols . get (LE)) ; p . field_hex ("SizeOfOptionalHeader" , header . size_of_optional_header . get (LE) ,) ; p . field_hex ("Characteristics" , header . characteristics . get (LE)) ; p . flags (header . characteristics . get (LE) , 0 , FLAGS_IMAGE_FILE) ; }) ; }
};
}
