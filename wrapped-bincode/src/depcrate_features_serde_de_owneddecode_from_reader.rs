// Generated macro for decode_from_reader (function)
macro_rules! Depcrate_features_serde_de_owneddecode_from_reader {
() => {
// Module: crate::features::serde::de_owned
// Provides: {"decode_from_reader"}
// Dependencies: {}
# [doc = " Attempt to decode a given type `D` from the given [Reader]."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: ../config/index.html"] pub fn decode_from_reader < D : DeserializeOwned , R : Reader , C : Config > (reader : R , config : C ,) -> Result < D , DecodeError > { let mut serde_decoder = OwnedSerdeDecoder :: < DecoderImpl < R , C , () > > :: from_reader (reader , config) ; D :: deserialize (serde_decoder . as_deserializer ()) }
};
}
