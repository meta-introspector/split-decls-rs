// Generated macro for impl_221 (impl)
macro_rules! Depcrate_features_serdeimpl_221 {
() => {
// Module: crate::features::serde
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'de , T , Context > crate :: BorrowDecode < 'de , Context > for Compat < T > where T : serde :: de :: DeserializeOwned , { fn borrow_decode < D : crate :: de :: BorrowDecoder < 'de > > (decoder : & mut D ,) -> Result < Self , crate :: error :: DecodeError > { let serde_decoder = de_owned :: SerdeDecoder { de : decoder } ; T :: deserialize (serde_decoder) . map (Compat) } }
};
}
