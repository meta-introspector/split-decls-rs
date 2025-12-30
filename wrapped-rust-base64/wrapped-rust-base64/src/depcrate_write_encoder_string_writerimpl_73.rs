// Generated macro for impl_73 (impl)
macro_rules! Depcrate_write_encoder_string_writerimpl_73 {
() => {
// Module: crate::write::encoder_string_writer
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'e , E : Engine , S : StrConsumer > EncoderStringWriter < 'e , E , S > { # [doc = " Create a `EncoderStringWriter` that will append to the provided `StrConsumer`."] pub fn from_consumer (str_consumer : S , engine : & 'e E) -> Self { EncoderStringWriter { encoder : EncoderWriter :: new (Utf8SingleCodeUnitWriter { str_consumer } , engine) , } } # [doc = " Encode all remaining buffered data, including any trailing incomplete input triples and"] # [doc = " associated padding."] # [doc = ""] # [doc = " Returns the base64-encoded form of the accumulated written data."] pub fn into_inner (mut self) -> S { self . encoder . finish () . expect ("Writing to a consumer should never fail") . str_consumer } }
};
}
