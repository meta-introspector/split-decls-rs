// Generated macro for impl_226 (impl)
macro_rules! Depcrate_enc_encoderimpl_226 {
() => {
// Module: crate::enc::encoder
// Provides: {"impl_226"}
// Dependencies: {}
impl LzmaEncoderTrait for LzmaEncoderModes { fn get_next_symbol (& mut self , encoder : & mut LzmaEncoder) -> u32 { match self { LzmaEncoderModes :: Fast (a) => a . get_next_symbol (encoder) , LzmaEncoderModes :: Normal (a) => a . get_next_symbol (encoder) , } } fn reset (& mut self) { match self { LzmaEncoderModes :: Fast (a) => a . reset () , LzmaEncoderModes :: Normal (a) => a . reset () , } } }
};
}
