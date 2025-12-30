// Generated macro for impl_88 (impl)
macro_rules! Depcrate_features_impl_allocimpl_88 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for Rc < [T] > where T : BorrowDecode < 'de , Context > + 'de , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let vec = Vec :: borrow_decode (decoder) ? ; Ok (vec . into ()) } }
};
}
