// Generated macro for write_string_table (function)
macro_rules! Depcrate_coff_import_filewrite_string_table {
() => {
// Module: crate::coff_import_file
// Provides: {"write_string_table"}
// Dependencies: {}
fn write_string_table (b : & mut Vec < u8 > , strings : & [& [u8]]) -> Result < () > { let offset = b . len () ; b . extend (0u32 . to_le_bytes ()) ; for s in strings { b . write_all (s) ? ; b . write_all (& [0]) ? ; } let length : u32 = (b . len () - offset) . try_into () . unwrap () ; b [offset .. offset + size_of :: < u32 > ()] . copy_from_slice (& length . to_le_bytes ()) ; Ok (()) }
};
}
