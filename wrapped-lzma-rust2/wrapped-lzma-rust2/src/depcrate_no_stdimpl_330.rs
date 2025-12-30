// Generated macro for impl_330 (impl)
macro_rules! Depcrate_no_stdimpl_330 {
() => {
// Module: crate::no_std
// Provides: {"impl_330"}
// Dependencies: {}
impl < W : Write + ? Sized > Write for alloc :: boxed :: Box < W > { # [inline (always)] fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { (* * self) . write (buf) } # [inline (always)] fn flush (& mut self) -> crate :: Result < () > { (* * self) . flush () } }
};
}
