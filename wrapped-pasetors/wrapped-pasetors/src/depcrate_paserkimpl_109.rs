// Generated macro for impl_109 (impl)
macro_rules! Depcrate_paserkimpl_109 {
() => {
// Module: crate::paserk
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (feature = "v4")] impl TryFrom < & str > for AsymmetricPublicKey < V4 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k4" , "public" , V4 :: PUBLIC_KEY) ? , phantom : PhantomData , }) } }
};
}
