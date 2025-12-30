// Generated macro for impl_480 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_480 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_480"}
// Dependencies: {}
impl TryFrom < & PrivateKey > for PublicKey { type Error = UnknownCryptoError ; fn try_from (private_key : & PrivateKey) -> Result < Self , Self :: Error > { let scalar = Scalar :: from_slice (private_key . unprotected_as_bytes ()) ? ; Ok (PublicKey :: from (mont_ladder (& scalar , FieldElement :: from_bytes (& BASEPOINT)) . as_bytes () ,)) } }
};
}
