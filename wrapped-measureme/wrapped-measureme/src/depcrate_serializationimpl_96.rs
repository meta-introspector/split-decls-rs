// Generated macro for impl_96 (impl)
macro_rules! Depcrate_serializationimpl_96 {
() => {
// Module: crate::serialization
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a > Write for StdWriteAdapter < 'a > { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . 0 . write_bytes_atomic (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { let mut data = self . 0 . data . lock () ; let SerializationSinkInner { ref mut buffer , addr : _ , } = * data ; self . 0 . flush (buffer) ; self . 0 . shared_state . 0 . lock () . flush () ? ; Ok (()) } }
};
}
