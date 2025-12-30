// Generated macro for impl_107 (impl)
macro_rules! Depcrate_paserkimpl_107 {
() => {
// Module: crate::paserk
// Provides: {"impl_107"}
// Dependencies: {}
# [cfg (feature = "v3")] impl TryFrom < & str > for AsymmetricPublicKey < V3 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k3" , "public" , V3 :: PUBLIC_KEY) ? , phantom : PhantomData , }) } }
};
}
