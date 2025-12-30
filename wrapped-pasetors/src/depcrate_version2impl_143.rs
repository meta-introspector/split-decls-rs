// Generated macro for impl_143 (impl)
macro_rules! Depcrate_version2impl_143 {
() => {
// Module: crate::version2
// Provides: {"impl_143"}
// Dependencies: {}
impl Generate < AsymmetricKeyPair < V2 > , V2 > for AsymmetricKeyPair < V2 > { fn generate () -> Result < AsymmetricKeyPair < V2 > , Error > { let key_pair = KeyPair :: generate () ; let secret = AsymmetricSecretKey :: < V2 > :: from (key_pair . sk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; let public = AsymmetricPublicKey :: < V2 > :: from (key_pair . pk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; Ok (Self { public , secret }) } }
};
}
