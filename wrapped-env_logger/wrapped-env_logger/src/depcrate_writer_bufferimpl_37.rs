// Generated macro for impl_37 (impl)
macro_rules! Depcrate_writer_bufferimpl_37 {
() => {
// Module: crate::writer::buffer
// Provides: {"impl_37"}
// Dependencies: {}
impl Buffer { pub (crate) fn clear (& mut self) { self . 0 . clear () ; } pub (crate) fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . extend (buf) ; Ok (buf . len ()) } pub (crate) fn flush (& mut self) -> io :: Result < () > { Ok (()) } pub (crate) fn as_bytes (& self) -> & [u8] { & self . 0 } }
};
}
