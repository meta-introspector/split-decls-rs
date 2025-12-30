// Generated macro for impl_220 (impl)
macro_rules! Depcrate_features_serdeimpl_220 {
() => {
// Module: crate::features::serde
// Provides: {"impl_220"}
// Dependencies: {}
impl < Context , T > crate :: Decode < Context > for Compat < T > where T : serde :: de :: DeserializeOwned , { fn decode < D : crate :: de :: Decoder > (decoder : & mut D) -> Result < Self , crate :: error :: DecodeError > { let serde_decoder = de_owned :: SerdeDecoder { de : decoder } ; T :: deserialize (serde_decoder) . map (Compat) } }
};
}
