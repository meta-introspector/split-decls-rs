// Generated macro for impl_125 (impl)
macro_rules! Depcrate_features_impl_stdimpl_125 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'de , Context > BorrowDecode < 'de , Context > for & 'de Path { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let str = < & 'de str > :: borrow_decode (decoder) ? ; Ok (Path :: new (str)) } }
};
}
