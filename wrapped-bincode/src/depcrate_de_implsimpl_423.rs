// Generated macro for impl_423 (impl)
macro_rules! Depcrate_de_implsimpl_423 {
() => {
// Module: crate::de::impls
// Provides: {"impl_423"}
// Dependencies: {}
impl < 'a , 'de : 'a , Context > BorrowDecode < 'de , Context > for & 'a str { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let slice = < & [u8] > :: borrow_decode (decoder) ? ; core :: str :: from_utf8 (slice) . map_err (| inner | DecodeError :: Utf8 { inner }) } }
};
}
