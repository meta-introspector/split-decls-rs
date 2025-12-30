// Generated macro for impl_417 (impl)
macro_rules! Depcrate_de_implsimpl_417 {
() => {
// Module: crate::de::impls
// Provides: {"impl_417"}
// Dependencies: {}
impl < 'de , Context , T : BorrowDecode < 'de , Context > > BorrowDecode < 'de , Context > for Wrapping < T > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Wrapping (T :: borrow_decode (decoder) ?)) } }
};
}
