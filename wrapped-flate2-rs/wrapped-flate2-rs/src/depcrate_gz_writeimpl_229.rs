// Generated macro for impl_229 (impl)
macro_rules! Depcrate_gz_writeimpl_229 {
() => {
// Module: crate::gz::write
// Provides: {"impl_229"}
// Dependencies: {}
impl < W : Write > Write for GzEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { assert_eq ! (self . crc_bytes_written , 0) ; self . write_header () ? ; let n = self . inner . write (buf) ? ; self . crc . update (& buf [.. n]) ; Ok (n) } fn flush (& mut self) -> io :: Result < () > { assert_eq ! (self . crc_bytes_written , 0) ; self . write_header () ? ; self . inner . flush () } }
};
}
