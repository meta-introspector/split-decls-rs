// Generated macro for impl_475 (impl)
macro_rules! Depcrate_hashimpl_475 {
() => {
// Module: crate::hash
// Provides: {"impl_475"}
// Dependencies: {}
impl Write for Hasher { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . update (buf) ? ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
