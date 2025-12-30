// Generated macro for impl_329 (impl)
macro_rules! Depcrate_no_stdimpl_329 {
() => {
// Module: crate::no_std
// Provides: {"impl_329"}
// Dependencies: {}
impl < R : Read + ? Sized > Read for alloc :: boxed :: Box < R > { # [inline (always)] fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { (* * self) . read (buf) } # [inline (always)] fn read_exact (& mut self , buf : & mut [u8]) -> crate :: Result < () > { (* * self) . read_exact (buf) } }
};
}
