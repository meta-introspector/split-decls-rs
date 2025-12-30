// Generated macro for impl_226 (impl)
macro_rules! Depcrate_features_serdeimpl_226 {
() => {
// Module: crate::features::serde
// Provides: {"impl_226"}
// Dependencies: {}
impl < 'de , T , Context > crate :: de :: BorrowDecode < 'de , Context > for BorrowCompat < T > where T : serde :: de :: Deserialize < 'de > , { fn borrow_decode < D : crate :: de :: BorrowDecoder < 'de > > (decoder : & mut D ,) -> Result < Self , crate :: error :: DecodeError > { let serde_decoder = de_borrowed :: SerdeDecoder { de : decoder , pd : core :: marker :: PhantomData , } ; T :: deserialize (serde_decoder) . map (BorrowCompat) } }
};
}
