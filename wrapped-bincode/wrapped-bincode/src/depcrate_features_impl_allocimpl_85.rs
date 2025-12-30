// Generated macro for impl_85 (impl)
macro_rules! Depcrate_features_impl_allocimpl_85 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'de , Context > BorrowDecode < 'de , Context > for Rc < str > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let decoded = String :: decode (decoder) ? ; Ok (decoded . into ()) } }
};
}
