// Generated macro for coded_index_size (function)
macro_rules! Depcrate_reader_filecoded_index_size {
() => {
// Module: crate::reader::file
// Provides: {"coded_index_size"}
// Dependencies: {}
fn coded_index_size (tables : & [usize]) -> usize { fn small (row_count : usize , bits : u8) -> bool { (row_count as u64) < (1u64 << (16 - bits)) } fn bits_needed (value : usize) -> u8 { let mut value = value - 1 ; let mut bits : u8 = 1 ; while { value >>= 1 ; value != 0 } { bits += 1 ; } bits } let bits_needed = bits_needed (tables . len ()) ; if tables . iter () . all (| table | small (* table , bits_needed)) { 2 } else { 4 } }
};
}
