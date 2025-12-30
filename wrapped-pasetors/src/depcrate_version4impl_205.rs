// Generated macro for impl_205 (impl)
macro_rules! Depcrate_version4impl_205 {
() => {
// Module: crate::version4
// Provides: {"impl_205"}
// Dependencies: {}
impl Generate < AsymmetricKeyPair < V4 > , V4 > for AsymmetricKeyPair < V4 > { fn generate () -> Result < AsymmetricKeyPair < V4 > , Error > { let key_pair = KeyPair :: generate () ; let secret = AsymmetricSecretKey :: < V4 > :: from (key_pair . sk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; let public = AsymmetricPublicKey :: < V4 > :: from (key_pair . pk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; Ok (Self { public , secret }) } }
};
}
