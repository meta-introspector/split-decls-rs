// Generated macro for impl_75 (impl)
macro_rules! Depcrate_write_encoder_string_writerimpl_75 {
() => {
// Module: crate::write::encoder_string_writer
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'e , E : Engine , S : StrConsumer > io :: Write for EncoderStringWriter < 'e , E , S > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . encoder . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . encoder . flush () } }
};
}
