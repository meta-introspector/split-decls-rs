// Generated macro for impl_74 (impl)
macro_rules! Depcrate_features_impl_allocimpl_74 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for Box < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Box :: new (t)) } }
};
}
