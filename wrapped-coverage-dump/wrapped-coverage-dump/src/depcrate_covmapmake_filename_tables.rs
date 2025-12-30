// Generated macro for make_filename_tables (function)
macro_rules! Depcrate_covmapmake_filename_tables {
() => {
// Module: crate::covmap
// Provides: {"make_filename_tables"}
// Dependencies: {}
pub (crate) fn make_filename_tables (llvm_ir : & str) -> anyhow :: Result < FilenameTables > { let mut map = HashMap :: default () ; for line in llvm_ir . lines () . filter (| line | is_covmap_line (line)) { let CovmapLineData { payload } = parse_covmap_line (line) ? ; let mut parser = Parser :: new (& payload) ; let n_filenames = parser . read_uleb128_usize () ? ; let uncompressed_bytes = parser . read_chunk_to_uncompressed_bytes () ? ; parser . ensure_empty () ? ; let mut filenames_table = vec ! [] ; let mut parser = Parser :: new (& uncompressed_bytes) ; for _ in 0 .. n_filenames { let len = parser . read_uleb128_usize () ? ; let bytes = parser . read_n_bytes (len) ? ; let filename = str :: from_utf8 (bytes) ? ; filenames_table . push (filename . to_owned ()) ; } let filenames_hash = truncated_md5 (& payload) ; map . insert (filenames_hash , filenames_table) ; } Ok (FilenameTables { map }) }
};
}
