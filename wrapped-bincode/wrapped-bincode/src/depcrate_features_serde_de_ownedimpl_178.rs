// Generated macro for impl_178 (impl)
macro_rules! Depcrate_features_serde_de_ownedimpl_178 {
() => {
// Module: crate::features::serde::de_owned
// Provides: {"impl_178"}
// Dependencies: {}
impl < DE : Decoder > OwnedSerdeDecoder < DE > { # [doc = " Return a type implementing `serde::Deserializer`."] pub fn as_deserializer < 'a > (& 'a mut self ,) -> impl for < 'de > serde :: Deserializer < 'de , Error = DecodeError > + 'a { SerdeDecoder { de : & mut self . de } } }
};
}
