// Generated macro for impl_74 (impl)
macro_rules! Depcrate_write_encoder_string_writerimpl_74 {
() => {
// Module: crate::write::encoder_string_writer
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'e , E : Engine > EncoderStringWriter < 'e , E , String > { # [doc = " Create a `EncoderStringWriter` that will encode into a new `String` with the provided config."] pub fn new (engine : & 'e E) -> Self { EncoderStringWriter :: from_consumer (String :: new () , engine) } }
};
}
