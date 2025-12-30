// Generated macro for impl_316 (impl)
macro_rules! Depcrate_lzma2_reader_mtimpl_316 {
() => {
// Module: crate::lzma2_reader_mt
// Provides: {"impl_316"}
// Dependencies: {}
impl < R : Read > Read for Lzma2ReaderMt < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if buf . is_empty () { return Ok (0) ; } let bytes_read = self . current_chunk . read (buf) ? ; if bytes_read > 0 { return Ok (bytes_read) ; } let chunk_data = self . get_next_uncompressed_chunk () ? ; let Some (chunk_data) = chunk_data else { return Ok (0) ; } ; self . current_chunk = Cursor :: new (chunk_data) ; self . read (buf) } }
};
}
