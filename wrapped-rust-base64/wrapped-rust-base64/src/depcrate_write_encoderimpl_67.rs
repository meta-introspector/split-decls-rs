// Generated macro for impl_67 (impl)
macro_rules! Depcrate_write_encoderimpl_67 {
() => {
// Module: crate::write::encoder
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'e , E : Engine , W : io :: Write > Drop for EncoderWriter < 'e , E , W > { fn drop (& mut self) { if ! self . panicked { let _ = self . write_final_leftovers () ; } } }
};
}
