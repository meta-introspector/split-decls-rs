// Generated macro for impl_323 (impl)
macro_rules! Depcrate_no_stdimpl_323 {
() => {
// Module: crate::no_std
// Provides: {"impl_323"}
// Dependencies: {}
impl < R : Read > Read for & mut R { # [inline (always)] fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { (* * self) . read (buf) } # [inline (always)] fn read_exact (& mut self , buf : & mut [u8]) -> crate :: Result < () > { (* * self) . read_exact (buf) } }
};
}
