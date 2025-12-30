// Generated macro for impl_95 (impl)
macro_rules! Depcrate_features_impl_allocimpl_95 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_95"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < 'de , T , Context > BorrowDecode < 'de , Context > for Arc < [T] > where T : BorrowDecode < 'de , Context > + 'de , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let vec = Vec :: borrow_decode (decoder) ? ; Ok (vec . into ()) } }
};
}
