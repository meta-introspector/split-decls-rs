// Generated macro for impl_441 (impl)
macro_rules! Depcrate_de_implsimpl_441 {
() => {
// Module: crate::de::impls
// Provides: {"impl_441"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for Range < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let min = T :: borrow_decode (decoder) ? ; let max = T :: borrow_decode (decoder) ? ; Ok (min .. max) } }
};
}
