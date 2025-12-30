// Generated macro for impl_54 (impl)
macro_rules! Depcrate_features_impl_allocimpl_54 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for BinaryHeap < T > where T : BorrowDecode < 'de , Context > + Ord , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: borrow_decode (decoder) ? . into ()) } }
};
}
