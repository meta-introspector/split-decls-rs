// Generated macro for impl_324 (impl)
macro_rules! Depcrate_de_decoderimpl_324 {
() => {
// Module: crate::de::decoder
// Provides: {"impl_324"}
// Dependencies: {}
impl < R : Reader , C : Config , Context > DecoderImpl < R , C , Context > { # [doc = " Construct a new Decoder"] pub fn new (reader : R , config : C , context : Context) -> DecoderImpl < R , C , Context > { DecoderImpl { reader , config , bytes_read : 0 , context , } } }
};
}
