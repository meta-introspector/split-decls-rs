// Generated macro for impl_171 (impl)
macro_rules! Depcrate_version3impl_171 {
() => {
// Module: crate::version3
// Provides: {"impl_171"}
// Dependencies: {}
impl Generate < AsymmetricKeyPair < V3 > , V3 > for AsymmetricKeyPair < V3 > { fn generate () -> Result < AsymmetricKeyPair < V3 > , Error > { let key = SigningKey :: random (& mut OsRng) ; let public = AsymmetricPublicKey :: < V3 > :: from (VerifyingKey :: from (& key) . to_encoded_point (true) . as_ref () ,) ? ; let secret = AsymmetricSecretKey :: < V3 > :: from (key . to_bytes () . as_slice ()) ? ; Ok (Self { public , secret }) } }
};
}
