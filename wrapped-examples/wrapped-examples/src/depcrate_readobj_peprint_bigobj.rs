// Generated macro for print_bigobj (function)
macro_rules! Depcrate_readobj_peprint_bigobj {
() => {
// Module: crate::readobj::pe
// Provides: {"print_bigobj"}
// Dependencies: {}
fn print_bigobj (p : & mut Printer < '_ > , header : & AnonObjectHeaderBigobj) { if ! p . options . file { return ; } p . group ("AnonObjectHeaderBigObj" , | p | { p . field_hex ("Signature1" , header . sig1 . get (LE)) ; p . field_hex ("Signature2" , header . sig2 . get (LE)) ; p . field ("Version" , header . version . get (LE)) ; p . field_enum ("Machine" , header . machine . get (LE) , FLAGS_IMAGE_FILE_MACHINE) ; p . field ("TimeDateStamp" , header . time_date_stamp . get (LE)) ; p . field ("ClassId" , format ! ("{:08X}-{:04X}-{:04X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}" , header . class_id . data1 () . get (LE) , header . class_id . data2 () . get (LE) , header . class_id . data3 () . get (LE) , header . class_id . data4 () [0] , header . class_id . data4 () [1] , header . class_id . data4 () [2] , header . class_id . data4 () [3] , header . class_id . data4 () [4] , header . class_id . data4 () [5] , header . class_id . data4 () [6] , header . class_id . data4 () [7] ,) ,) ; p . field_hex ("SizeOfData" , header . size_of_data . get (LE)) ; p . field_hex ("Flags" , header . flags . get (LE)) ; p . field_hex ("MetaDataSize" , header . meta_data_size . get (LE)) ; p . field_hex ("MetaDataOffset" , header . meta_data_offset . get (LE)) ; p . field ("NumberOfSections" , header . number_of_sections . get (LE)) ; p . field_hex ("PointerToSymbolTable" , header . pointer_to_symbol_table . get (LE) ,) ; p . field ("NumberOfSymbols" , header . number_of_symbols . get (LE)) ; }) ; }
};
}
