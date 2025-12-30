// Generated macro for load_root_certs (function)
macro_rules! Depcrateload_root_certs {
() => {
// Module: crate
// Provides: {"load_root_certs"}
// Dependencies: {}
fn load_root_certs (filename : & str) -> Arc < RootCertStore > { let mut roots = RootCertStore :: empty () ; let filename = match filename { "" => "../../../../../test-ca/rsa-2048/ca.cert" , filename => filename , } ; roots . add_parsable_certificates (CertificateDer :: pem_file_iter (filename) . unwrap () . map (| item | item . unwrap ()) ,) ; Arc :: new (roots) }
};
}
