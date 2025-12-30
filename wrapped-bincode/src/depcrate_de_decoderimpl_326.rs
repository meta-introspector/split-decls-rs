// Generated macro for impl_326 (impl)
macro_rules! Depcrate_de_decoderimpl_326 {
() => {
// Module: crate::de::decoder
// Provides: {"impl_326"}
// Dependencies: {}
impl < 'de , R : BorrowReader < 'de > , C : Config , Context > BorrowDecoder < 'de > for DecoderImpl < R , C , Context > { type BR = R ; fn borrow_reader (& mut self) -> & mut Self :: BR { & mut self . reader } }
};
}
