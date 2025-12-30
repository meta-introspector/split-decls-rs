// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < W > Write for TermionBackend < W > where W : Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . writer . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
};
}
