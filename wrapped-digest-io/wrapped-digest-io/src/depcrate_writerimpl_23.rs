// Generated macro for impl_23 (impl)
macro_rules! Depcrate_writerimpl_23 {
() => {
// Module: crate::writer
// Provides: {"impl_23"}
// Dependencies: {}
impl < D : Digest , W : io :: Write > io :: Write for HashWriter < D , W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let bytes = self . writer . write (buf) ? ; if bytes > 0 { self . hasher . update (& buf [0 .. bytes]) ; } Ok (bytes) } fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
};
}
