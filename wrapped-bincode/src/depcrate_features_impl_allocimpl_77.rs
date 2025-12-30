// Generated macro for impl_77 (impl)
macro_rules! Depcrate_features_impl_allocimpl_77 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for Box < [T] > where T : BorrowDecode < 'de , Context > + 'de , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let vec = Vec :: borrow_decode (decoder) ? ; Ok (vec . into_boxed_slice ()) } }
};
}
