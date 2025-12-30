// Generated macro for impl_79 (impl)
macro_rules! Depcrate_features_impl_allocimpl_79 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'cow , T , Context > BorrowDecode < 'cow , Context > for Cow < 'cow , T > where T : ToOwned + ? Sized , & 'cow T : BorrowDecode < 'cow , Context > , { fn borrow_decode < D : BorrowDecoder < 'cow , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = < & T > :: borrow_decode (decoder) ? ; Ok (Cow :: Borrowed (t)) } }
};
}
