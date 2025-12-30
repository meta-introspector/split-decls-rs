// Generated macro for impl_6 (impl)
macro_rules! Depcrate_io_wrapperimpl_6 {
() => {
// Module: crate::io_wrapper
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : XofReader > io :: Read for IoWrapper < T > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { XofReader :: read (& mut self . 0 , buf) ; Ok (buf . len ()) } }
};
}
