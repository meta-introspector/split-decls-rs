// Generated macro for impl_103 (impl)
macro_rules! Depcrate_paserkimpl_103 {
() => {
// Module: crate::paserk
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (feature = "v4")] impl TryFrom < & str > for AsymmetricSecretKey < V4 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { let mut buf = validate_paserk_string (value , "k4" , "secret" , V4 :: SECRET_KEY) ? ; let ret = Self :: from (& buf) ? ; buf . iter_mut () . zeroize () ; Ok (ret) } }
};
}
