// Generated macro for impl_120 (impl)
macro_rules! Depcrate_features_impl_stdimpl_120 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for RwLock < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (RwLock :: new (t)) } }
};
}
