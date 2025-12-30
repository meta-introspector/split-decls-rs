// Generated macro for impl_836 (impl)
macro_rules! Depcrate_signimpl_836 {
() => {
// Module: crate::sign
// Provides: {"impl_836"}
// Dependencies: {}
impl Write for Signer < '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . update (buf) ? ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
