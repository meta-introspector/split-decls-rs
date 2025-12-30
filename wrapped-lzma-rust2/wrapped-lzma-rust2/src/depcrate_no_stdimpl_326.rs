// Generated macro for impl_326 (impl)
macro_rules! Depcrate_no_stdimpl_326 {
() => {
// Module: crate::no_std
// Provides: {"impl_326"}
// Dependencies: {}
impl < W : Write > Write for & mut W { # [inline (always)] fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { (* * self) . write (buf) } # [inline (always)] fn flush (& mut self) -> crate :: Result < () > { (* * self) . flush () } }
};
}
