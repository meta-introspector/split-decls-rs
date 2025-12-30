// Generated macro for impl_328 (impl)
macro_rules! Depcrate_no_stdimpl_328 {
() => {
// Module: crate::no_std
// Provides: {"impl_328"}
// Dependencies: {}
impl Write for Vec < u8 > { # [inline (always)] fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline (always)] fn flush (& mut self) -> crate :: Result < () > { Ok (()) } }
};
}
