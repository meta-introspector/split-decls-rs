// Generated macro for impl_72 (impl)
macro_rules! Depcrate_keysimpl_72 {
() => {
// Module: crate::keys
// Provides: {"impl_72"}
// Dependencies: {}
impl < V : Version > AsymmetricPublicKey < V > { # [doc = " Create a `AsymmetricPublicKey` from `bytes`."] pub fn from (bytes : & [u8]) -> Result < Self , Error > { V :: validate_public_key (bytes) ? ; Ok (Self { bytes : bytes . to_vec () , phantom : PhantomData , }) } # [doc = " Return this as a byte-slice."] pub fn as_bytes (& self) -> & [u8] { self . bytes . as_slice () } }
};
}
