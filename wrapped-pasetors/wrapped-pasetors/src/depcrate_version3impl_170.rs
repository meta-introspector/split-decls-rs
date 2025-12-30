// Generated macro for impl_170 (impl)
macro_rules! Depcrate_version3impl_170 {
() => {
// Module: crate::version3
// Provides: {"impl_170"}
// Dependencies: {}
impl TryFrom < & AsymmetricSecretKey < V3 > > for AsymmetricPublicKey < V3 > { type Error = Error ; fn try_from (value : & AsymmetricSecretKey < V3 >) -> Result < Self , Self :: Error > { let sk = SigningKey :: from_bytes (value . as_bytes () . into ()) . map_err (| _ | Error :: Key) ? ; AsymmetricPublicKey :: < V3 > :: from (sk . verifying_key () . to_encoded_point (true) . as_bytes ()) } }
};
}
