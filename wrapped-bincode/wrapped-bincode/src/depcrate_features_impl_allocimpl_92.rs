// Generated macro for impl_92 (impl)
macro_rules! Depcrate_features_impl_allocimpl_92 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_92"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl < 'de , Context > BorrowDecode < 'de , Context > for Arc < str > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let decoded = String :: decode (decoder) ? ; Ok (decoded . into ()) } }
};
}
