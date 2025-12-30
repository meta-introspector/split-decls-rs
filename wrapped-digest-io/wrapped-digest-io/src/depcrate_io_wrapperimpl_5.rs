// Generated macro for impl_5 (impl)
macro_rules! Depcrate_io_wrapperimpl_5 {
() => {
// Module: crate::io_wrapper
// Provides: {"impl_5"}
// Dependencies: {}
impl < T : Update > io :: Write for IoWrapper < T > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { Update :: update (& mut self . 0 , buf) ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
