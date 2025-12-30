// Generated macro for print_coff_import (function)
macro_rules! Depcrate_readobj_peprint_coff_import {
() => {
// Module: crate::readobj::pe
// Provides: {"print_coff_import"}
// Dependencies: {}
pub (super) fn print_coff_import (p : & mut Printer < '_ > , data : & [u8]) { let mut offset = 0 ; if let Some (header) = ImportObjectHeader :: parse (data , & mut offset) . print_err (p) { writeln ! (p . w () , "Format: COFF import") . unwrap () ; if ! p . options . file { return ; } p . group ("ImportObjectHeader" , | p | { p . field_hex ("Signature1" , header . sig1 . get (LE)) ; p . field_hex ("Signature2" , header . sig2 . get (LE)) ; p . field ("Version" , header . version . get (LE)) ; p . field_enum ("Machine" , header . machine . get (LE) , FLAGS_IMAGE_FILE_MACHINE) ; p . field ("TimeDateStamp" , header . time_date_stamp . get (LE)) ; p . field_hex ("SizeOfData" , header . size_of_data . get (LE)) ; p . field ("OrdinalOrHint" , header . ordinal_or_hint . get (LE)) ; p . field_enum ("ImportType" , header . import_type () , FLAGS_IMAGE_OBJECT_TYPE) ; p . field_enum ("NameType" , header . name_type () , FLAGS_IMAGE_OBJECT_NAME) ; if let Some (data) = header . parse_data (data , & mut offset) . print_err (p) { p . field_inline_string ("Symbol" , data . symbol ()) ; p . field_inline_string ("Dll" , data . dll ()) ; if let Some (export) = data . export () { p . field_inline_string ("Export" , export) ; } } }) ; } }
};
}
