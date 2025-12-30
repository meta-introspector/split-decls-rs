// Generated macro for impl_419 (impl)
macro_rules! Depcrate_de_implsimpl_419 {
() => {
// Module: crate::de::impls
// Provides: {"impl_419"}
// Dependencies: {}
impl < 'de , Context , T : BorrowDecode < 'de , Context > > BorrowDecode < 'de , Context > for Reverse < T > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Reverse (T :: borrow_decode (decoder) ?)) } }
};
}
