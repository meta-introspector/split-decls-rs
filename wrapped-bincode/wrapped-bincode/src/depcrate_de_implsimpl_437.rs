// Generated macro for impl_437 (impl)
macro_rules! Depcrate_de_implsimpl_437 {
() => {
// Module: crate::de::impls
// Provides: {"impl_437"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for RefCell < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (RefCell :: new (t)) } }
};
}
