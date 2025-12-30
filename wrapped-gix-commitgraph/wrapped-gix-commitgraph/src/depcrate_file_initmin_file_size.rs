// Generated macro for MIN_FILE_SIZE (const)
macro_rules! Depcrate_file_initMIN_FILE_SIZE {
() => {
// Module: crate::file::init
// Provides: {"MIN_FILE_SIZE"}
// Dependencies: {}
const MIN_FILE_SIZE : usize = HEADER_LEN + gix_chunk :: file :: Index :: size_for_entries (3) + FAN_LEN * 4 + gix_hash :: Kind :: shortest () . len_in_bytes () ;
};
}
