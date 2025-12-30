// Generated macro for impl_165 (impl)
macro_rules! Depcrate_features_serde_de_borrowedimpl_165 {
() => {
// Module: crate::features::serde::de_borrowed
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'de , C : Config , Context > BorrowedSerdeDecoder < 'de , DecoderImpl < SliceReader < 'de > , C , Context > > { # [doc = " Creates the decoder from a borrowed slice."] pub fn from_slice (slice : & 'de [u8] , config : C , context : Context ,) -> BorrowedSerdeDecoder < 'de , DecoderImpl < SliceReader < 'de > , C , Context > > where C : Config , { let reader = SliceReader :: new (slice) ; let decoder = DecoderImpl :: new (reader , config , context) ; Self { de : decoder , pd : PhantomData , } } }
};
}
