// Generated macro for impl_84 (impl)
macro_rules! Depcrate_features_impl_allocimpl_84 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for Rc < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Rc :: new (t)) } }
};
}
