// Generated macro for load_certs (function)
macro_rules! Depcrate_ffdhe_kx_with_opensslload_certs {
() => {
// Module: crate::ffdhe_kx_with_openssl
// Provides: {"load_certs"}
// Dependencies: {}
fn load_certs () -> Vec < CertificateDer < 'static > > { CertificateDer :: pem_file_iter (CERT_CHAIN_FILE) . unwrap () . map (| c | c . unwrap ()) . collect () }
};
}
