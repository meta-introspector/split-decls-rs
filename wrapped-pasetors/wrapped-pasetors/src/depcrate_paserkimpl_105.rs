// Generated macro for impl_105 (impl)
macro_rules! Depcrate_paserkimpl_105 {
() => {
// Module: crate::paserk
// Provides: {"impl_105"}
// Dependencies: {}
# [cfg (feature = "v2")] impl TryFrom < & str > for AsymmetricPublicKey < V2 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k2" , "public" , V2 :: PUBLIC_KEY) ? , phantom : PhantomData , }) } }
};
}
