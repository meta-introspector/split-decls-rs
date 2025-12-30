// Generated macro for impl_476 (impl)
macro_rules! Depcrate_enc_encoderimpl_476 {
() => {
// Module: crate::enc::encoder
// Provides: {"impl_476"}
// Dependencies: {}
impl < W : Writer , C : Config > Encoder for EncoderImpl < W , C > { type W = W ; type C = C ; # [inline] fn writer (& mut self) -> & mut Self :: W { & mut self . writer } # [inline] fn config (& self) -> & Self :: C { & self . config } }
};
}
