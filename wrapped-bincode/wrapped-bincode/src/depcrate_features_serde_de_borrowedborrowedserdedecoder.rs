// Generated macro for BorrowedSerdeDecoder (struct)
macro_rules! Depcrate_features_serde_de_borrowedBorrowedSerdeDecoder {
() => {
// Module: crate::features::serde::de_borrowed
// Provides: {"BorrowedSerdeDecoder"}
// Dependencies: {}
# [doc = " Serde decoder encapsulating a borrowed reader."] pub struct BorrowedSerdeDecoder < 'de , DE : BorrowDecoder < 'de > > { pub (super) de : DE , pub (super) pd : PhantomData < & 'de () > , }
};
}
