// Generated macro for impl_101 (impl)
macro_rules! Depcrate_paserkimpl_101 {
() => {
// Module: crate::paserk
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (feature = "v3")] impl TryFrom < & str > for AsymmetricSecretKey < V3 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { let buf = validate_paserk_string (value , "k3" , "secret" , V3 :: SECRET_KEY) ? ; let ret = Self { bytes : buf , phantom : PhantomData , } ; Ok (ret) } }
};
}
