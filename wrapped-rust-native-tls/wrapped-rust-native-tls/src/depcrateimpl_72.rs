// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < S : io :: Read + io :: Write > io :: Write for TlsStream < S > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } }
};
}
