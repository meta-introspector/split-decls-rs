// Generated macro for StrConsumer (trait)
macro_rules! Depcrate_write_encoder_string_writerStrConsumer {
() => {
// Module: crate::write::encoder_string_writer
// Provides: {"StrConsumer"}
// Dependencies: {}
# [doc = " An abstraction around consuming `str`s produced by base64 encoding."] pub trait StrConsumer { # [doc = " Consume the base64 encoded data in `buf`"] fn consume (& mut self , buf : & str) ; }
};
}
