// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl DummyClientAuth { fn new (trusted_cert_file : & str , mandatory : bool , root_hint_subjects : Arc < [DistinguishedName] > ,) -> Self { Self { mandatory , root_hint_subjects , parent : WebPkiClientVerifier :: builder (load_root_certs (trusted_cert_file) , & SelectedProvider :: from_env () . provider () ,) . build () . unwrap () , } } }
};
}
