// Generated macro for write_header (function)
macro_rules! Depcrate_pre_expandwrite_header {
() => {
// Module: crate::pre_expand
// Provides: {"write_header"}
// Dependencies: {}
fn write_header (source_file_name : & Path , source : & str , file : & mut File) { let mut hasher = SipHasher :: new () ; source . hash (& mut hasher) ; let source_hash = hasher . finish () ; for header_line in source . lines () . take_while (| line | line . starts_with ("//")) { writeln ! (file , "{}" , header_line) . unwrap () ; } writeln ! (file , r"
// This file is generated from {}
// source SipHash: {}
" , source_file_name . file_name () . unwrap () . to_string_lossy () , source_hash) . unwrap () ; }
};
}
