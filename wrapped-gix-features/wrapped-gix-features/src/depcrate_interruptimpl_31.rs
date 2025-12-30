// Generated macro for impl_31 (impl)
macro_rules! Depcrate_interruptimpl_31 {
() => {
// Module: crate::interrupt
// Provides: {"impl_31"}
// Dependencies: {}
impl < W > io :: Write for Write < '_ , W > where W : std :: io :: Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if self . should_interrupt . load (Ordering :: Relaxed) { return Err (std :: io :: Error :: other ("Interrupted")) ; } self . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
