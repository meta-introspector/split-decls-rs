// Generated macro for impl_32 (impl)
macro_rules! Depcrate_crcimpl_32 {
() => {
// Module: crate::crc
// Provides: {"impl_32"}
// Dependencies: {}
impl < W : Write > Write for CrcWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let amt = self . inner . write (buf) ? ; self . crc . update (& buf [.. amt]) ; Ok (amt) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
