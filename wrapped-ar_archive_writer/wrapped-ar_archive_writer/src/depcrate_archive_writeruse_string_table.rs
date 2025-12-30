// Generated macro for use_string_table (function)
macro_rules! Depcrate_archive_writeruse_string_table {
() => {
// Module: crate::archive_writer
// Provides: {"use_string_table"}
// Dependencies: {}
fn use_string_table (thin : bool , name : & str) -> bool { thin || name . len () >= 16 || name . contains ('/') }
};
}
