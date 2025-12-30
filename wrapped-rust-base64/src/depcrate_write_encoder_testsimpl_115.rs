// Generated macro for impl_115 (impl)
macro_rules! Depcrate_write_encoder_testsimpl_115 {
() => {
// Module: crate::write::encoder_tests
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a , W : Write , R : Rng > Write for InterruptingWriter < 'a , W , R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if self . rng . gen_range (0.0 .. 1.0) <= self . fraction { return Err (io :: Error :: new (io :: ErrorKind :: Interrupted , "interrupted")) ; } self . w . write (buf) } fn flush (& mut self) -> io :: Result < () > { if self . rng . gen_range (0.0 .. 1.0) <= self . fraction { return Err (io :: Error :: new (io :: ErrorKind :: Interrupted , "interrupted")) ; } self . w . flush () } }
};
}
