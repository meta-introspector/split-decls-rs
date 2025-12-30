// Generated macro for Utf8SingleCodeUnitWriter (struct)
macro_rules! Depcrate_write_encoder_string_writerUtf8SingleCodeUnitWriter {
() => {
// Module: crate::write::encoder_string_writer
// Provides: {"Utf8SingleCodeUnitWriter"}
// Dependencies: {}
# [doc = " A `Write` that only can handle bytes that are valid single-byte UTF-8 code units."] # [doc = ""] # [doc = " This is safe because we only use it when writing base64, which is always valid UTF-8."] struct Utf8SingleCodeUnitWriter < S : StrConsumer > { str_consumer : S , }
};
}
