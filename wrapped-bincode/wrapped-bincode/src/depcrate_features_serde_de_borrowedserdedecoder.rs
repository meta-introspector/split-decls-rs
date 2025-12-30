// Generated macro for SerdeDecoder (struct)
macro_rules! Depcrate_features_serde_de_borrowedSerdeDecoder {
() => {
// Module: crate::features::serde::de_borrowed
// Provides: {"SerdeDecoder"}
// Dependencies: {}
pub (super) struct SerdeDecoder < 'a , 'de , DE : BorrowDecoder < 'de > > { pub (super) de : & 'a mut DE , pub (super) pd : PhantomData < & 'de () > , }
};
}
