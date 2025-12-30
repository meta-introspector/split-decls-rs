// Generated macro for impl_475 (impl)
macro_rules! Depcrate_enc_encoderimpl_475 {
() => {
// Module: crate::enc::encoder
// Provides: {"impl_475"}
// Dependencies: {}
impl < W : Writer , C : Config > EncoderImpl < W , C > { # [doc = " Create a new Encoder"] pub const fn new (writer : W , config : C) -> EncoderImpl < W , C > { EncoderImpl { writer , config } } # [doc = " Return the underlying writer"] # [inline] pub fn into_writer (self) -> W { self . writer } }
};
}
