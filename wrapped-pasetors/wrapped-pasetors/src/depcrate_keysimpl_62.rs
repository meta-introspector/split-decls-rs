// Generated macro for impl_62 (impl)
macro_rules! Depcrate_keysimpl_62 {
() => {
// Module: crate::keys
// Provides: {"impl_62"}
// Dependencies: {}
impl < V : Version > SymmetricKey < V > { # [doc = " Create a `SymmetricKey` from `bytes`."] pub fn from (bytes : & [u8]) -> Result < Self , Error > { V :: validate_local_key (bytes) ? ; Ok (Self { bytes : bytes . to_vec () , phantom : PhantomData , }) } # [doc = " Return this as a byte-slice."] pub fn as_bytes (& self) -> & [u8] { self . bytes . as_slice () } }
};
}
