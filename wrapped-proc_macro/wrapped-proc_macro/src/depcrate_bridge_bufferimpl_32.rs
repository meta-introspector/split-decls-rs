// Generated macro for impl_32 (impl)
macro_rules! Depcrate_bridge_bufferimpl_32 {
() => {
// Module: crate::bridge::buffer
// Provides: {"impl_32"}
// Dependencies: {}
impl Write for Buffer { # [inline] fn write (& mut self , xs : & [u8]) -> io :: Result < usize > { self . extend_from_slice (xs) ; Ok (xs . len ()) } # [inline] fn write_all (& mut self , xs : & [u8]) -> io :: Result < () > { self . extend_from_slice (xs) ; Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
