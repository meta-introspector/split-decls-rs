// Generated macro for set_name_to_string_table_entry (function)
macro_rules! Depcrate_coff_import_fileset_name_to_string_table_entry {
() => {
// Module: crate::coff_import_file
// Provides: {"set_name_to_string_table_entry"}
// Dependencies: {}
fn set_name_to_string_table_entry (symbol : & mut ImageSymbol , offset : usize) { symbol . name [.. 4] . copy_from_slice (& [0 ; 4]) ; symbol . name [4 ..] . copy_from_slice (& u32 :: try_from (offset) . unwrap () . to_le_bytes ()) ; }
};
}
