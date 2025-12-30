// Generated macro for impl_180 (impl)
macro_rules! Depcrate_features_serde_de_ownedimpl_180 {
() => {
// Module: crate::features::serde::de_owned
// Provides: {"impl_180"}
// Dependencies: {}
impl < C : Config , R : Reader > OwnedSerdeDecoder < DecoderImpl < R , C , () > > { # [doc = " Creates the decoder from a [`Reader`] implementor."] pub fn from_reader (reader : R , config : C) -> OwnedSerdeDecoder < DecoderImpl < R , C , () > > where C : Config , { let decoder = DecoderImpl :: new (reader , config , ()) ; Self { de : decoder } } }
};
}
