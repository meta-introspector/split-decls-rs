// Generated macro for impl_36 (impl)
macro_rules! Depcrate_parseimpl_36 {
() => {
// Module: crate::parse
// Provides: {"impl_36"}
// Dependencies: {}
impl std :: io :: Write for TimeBuf { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . buf . extend_from_slice (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
