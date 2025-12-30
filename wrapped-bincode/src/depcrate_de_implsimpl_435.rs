// Generated macro for impl_435 (impl)
macro_rules! Depcrate_de_implsimpl_435 {
() => {
// Module: crate::de::impls
// Provides: {"impl_435"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for Cell < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Cell :: new (t)) } }
};
}
