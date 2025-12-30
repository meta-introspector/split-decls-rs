// Generated macro for impl_331 (impl)
macro_rules! Depcrate_de_decoderimpl_331 {
() => {
// Module: crate::de::decoder
// Provides: {"impl_331"}
// Dependencies: {}
impl < 'de , C , D : BorrowDecoder < 'de > > BorrowDecoder < 'de > for WithContext < '_ , D , C > { type BR = D :: BR ; fn borrow_reader (& mut self) -> & mut Self :: BR { self . decoder . borrow_reader () } }
};
}
