// Generated macro for impl_80 (impl)
macro_rules! Depcrate_write_encoder_string_writerimpl_80 {
() => {
// Module: crate::write::encoder_string_writer
// Provides: {"impl_80"}
// Dependencies: {}
impl < S : StrConsumer > io :: Write for Utf8SingleCodeUnitWriter < S > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let s = std :: str :: from_utf8 (buf) . expect ("Input must be valid UTF-8") ; self . str_consumer . consume (s) ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
