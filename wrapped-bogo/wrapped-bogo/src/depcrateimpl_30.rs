// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl Credential { fn load_from_file (& self , provider : & CryptoProvider) -> Credentials { let certs = CertificateDer :: pem_file_iter (& self . cert_file) . unwrap () . map (| cert | cert . unwrap ()) . collect :: < Vec < _ > > () ; let key = PrivateKeyDer :: from_pem_file (& self . key_file) . unwrap () ; Credentials :: from_der (Arc :: from (Identity :: from_cert_chain (certs) . unwrap ()) , key , provider ,) . unwrap () } fn configured (& self) -> bool { ! self . cert_file . is_empty () && ! self . key_file . is_empty () } }
};
}
