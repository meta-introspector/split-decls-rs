// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl DummyServerAuth { fn new (trusted_cert_file : & str , ocsp : OcspValidation) -> Self { Self { parent : WebPkiServerVerifier :: builder (load_root_certs (trusted_cert_file) , & SelectedProvider :: from_env () . provider () ,) . build () . unwrap () , ocsp , } } }
};
}
