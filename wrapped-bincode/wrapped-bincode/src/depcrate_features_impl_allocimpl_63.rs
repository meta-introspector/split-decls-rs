// Generated macro for impl_63 (impl)
macro_rules! Depcrate_features_impl_allocimpl_63 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for VecDeque < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: borrow_decode (decoder) ? . into ()) } }
};
}
