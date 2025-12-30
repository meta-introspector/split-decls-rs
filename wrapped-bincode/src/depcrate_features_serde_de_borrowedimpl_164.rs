// Generated macro for impl_164 (impl)
macro_rules! Depcrate_features_serde_de_borrowedimpl_164 {
() => {
// Module: crate::features::serde::de_borrowed
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'de , DE : BorrowDecoder < 'de > > BorrowedSerdeDecoder < 'de , DE > { # [doc = " Return a type implementing `serde::Deserializer`."] pub fn as_deserializer < 'a > (& 'a mut self ,) -> impl serde :: Deserializer < 'de , Error = DecodeError > + 'a { SerdeDecoder { de : & mut self . de , pd : PhantomData , } } }
};
}
