// Generated macro for impl_45 (impl)
macro_rules! Depcrate_stream_binary_readerimpl_45 {
() => {
// Module: crate::stream::binary_reader
// Provides: {"impl_45"}
// Dependencies: {}
impl < R : Read + Seek > PosReader < R > { fn read_all (& mut self , buf : & mut [u8]) -> Result < () , Error > { self . read_exact (buf) . map_err (| err | ErrorKind :: Io (err) . with_byte_offset (self . pos)) ? ; Ok (()) } fn seek (& mut self , pos : SeekFrom) -> Result < u64 , Error > { self . pos = self . reader . seek (pos) . map_err (| err | ErrorKind :: Io (err) . with_byte_offset (self . pos)) ? ; Ok (self . pos) } }
};
}
