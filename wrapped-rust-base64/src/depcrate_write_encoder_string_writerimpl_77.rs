// Generated macro for impl_77 (impl)
macro_rules! Depcrate_write_encoder_string_writerimpl_77 {
() => {
// Module: crate::write::encoder_string_writer
// Provides: {"impl_77"}
// Dependencies: {}
# [doc = " As for `io::Write`, `StrConsumer` is implemented automatically for `&mut S`."] impl < S : StrConsumer + ? Sized > StrConsumer for & mut S { fn consume (& mut self , buf : & str) { (* * self) . consume (buf) ; } }
};
}
