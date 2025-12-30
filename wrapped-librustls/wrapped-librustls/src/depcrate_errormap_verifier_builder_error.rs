// Generated macro for map_verifier_builder_error (function)
macro_rules! Depcrate_errormap_verifier_builder_error {
() => {
// Module: crate::error
// Provides: {"map_verifier_builder_error"}
// Dependencies: {}
pub (crate) fn map_verifier_builder_error (err : VerifierBuilderError) -> rustls_result { match err { VerifierBuilderError :: NoRootAnchors => { rustls_result :: ClientCertVerifierBuilderNoRootAnchors } VerifierBuilderError :: InvalidCrl (crl_err) => map_crl_error (crl_err) , _ => rustls_result :: General , } }
};
}
