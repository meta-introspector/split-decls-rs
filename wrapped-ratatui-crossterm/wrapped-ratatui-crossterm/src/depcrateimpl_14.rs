// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < W > Write for CrosstermBackend < W > where W : Write , { # [doc = " Writes a buffer of bytes to the underlying buffer."] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . writer . write (buf) } # [doc = " Flushes the underlying buffer."] fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
};
}
