// Generated macro for impl_125 (impl)
macro_rules! Depcrate_bufferimpl_125 {
() => {
// Module: crate::buffer
// Provides: {"impl_125"}
// Dependencies: {}
impl std :: io :: Write for Buffer { # [inline] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . 0 . extend (buf) ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
