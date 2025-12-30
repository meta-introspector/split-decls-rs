// Generated macro for impl_106 (impl)
macro_rules! Depcrateimpl_106 {
() => {
// Module: crate
// Provides: {"impl_106"}
// Dependencies: {}
impl ClientSideStepper < '_ > { fn make_config (params : & BenchmarkParams , resume : ResumptionKind) -> Arc < ClientConfig > { let cfg = ClientConfig :: builder (params . provider . clone ()) ; let mut cfg = match params . auth_key { AuthKeySource :: KeyType (key_type) => { let mut root_store = RootCertStore :: empty () ; root_store . add (key_type . ca_cert ()) . unwrap () ; cfg . with_root_certificates (root_store) . with_no_client_auth () . unwrap () } AuthKeySource :: FuzzingProvider => cfg . dangerous () . with_custom_certificate_verifier (rustls_fuzzing_provider :: server_verifier ()) . with_no_client_auth () . unwrap () , } ; if resume != ResumptionKind :: No { cfg . resumption = Resumption :: in_memory_sessions (128) ; } else { cfg . resumption = Resumption :: disabled () ; } Arc :: new (cfg) } }
};
}
