// Generated macro for impl_28 (impl)
macro_rules! Depcrate_interruptimpl_28 {
() => {
// Module: crate::interrupt
// Provides: {"impl_28"}
// Dependencies: {}
impl < R > io :: Read for Read < '_ , R > where R : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . should_interrupt . load (Ordering :: Relaxed) { return Err (std :: io :: Error :: other ("Interrupted")) ; } self . inner . read (buf) } }
};
}
