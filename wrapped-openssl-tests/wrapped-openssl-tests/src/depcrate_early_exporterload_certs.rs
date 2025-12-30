// Generated macro for load_certs (function)
macro_rules! Depcrate_early_exporterload_certs {
() => {
// Module: crate::early_exporter
// Provides: {"load_certs"}
// Dependencies: {}
fn load_certs () -> Vec < CertificateDer < 'static > > { CertificateDer :: pem_file_iter (CERT_CHAIN_FILE) . unwrap () . map (| c | c . unwrap ()) . collect () }
};
}
