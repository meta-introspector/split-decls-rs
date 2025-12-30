// Generated macro for impl_443 (impl)
macro_rules! Depcrate_de_implsimpl_443 {
() => {
// Module: crate::de::impls
// Provides: {"impl_443"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for RangeInclusive < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let min = T :: borrow_decode (decoder) ? ; let max = T :: borrow_decode (decoder) ? ; Ok (RangeInclusive :: new (min , max)) } }
};
}
