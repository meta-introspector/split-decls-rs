// Generated macro for impl_117 (impl)
macro_rules! Depcrate_write_encoder_testsimpl_117 {
() => {
// Module: crate::write::encoder_tests
// Provides: {"impl_117"}
// Dependencies: {}
impl < 'a , W : Write , R : Rng > Write for PartialInterruptingWriter < 'a , W , R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if self . rng . gen_range (0.0 .. 1.0) > self . no_interrupt_fraction { return Err (io :: Error :: new (io :: ErrorKind :: Interrupted , "interrupted")) ; } if self . rng . gen_range (0.0 .. 1.0) <= self . full_input_fraction || buf . is_empty () { self . w . write (buf) } else { self . w . write (& buf [0 .. (self . rng . gen_range (0 .. (buf . len () - 1)))]) } } fn flush (& mut self) -> io :: Result < () > { self . w . flush () } }
};
}
