// Generated macro for impl_91 (impl)
macro_rules! Depcrate_features_impl_allocimpl_91 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < 'de , T , Context > BorrowDecode < 'de , Context > for Arc < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Arc :: new (t)) } }
};
}
