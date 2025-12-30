// Generated macro for impl_842 (impl)
macro_rules! Depcrate_signimpl_842 {
() => {
// Module: crate::sign
// Provides: {"impl_842"}
// Dependencies: {}
impl Write for Verifier < '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . update (buf) ? ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
