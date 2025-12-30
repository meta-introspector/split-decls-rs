// Generated macro for impl_109 (impl)
macro_rules! Depcrateimpl_109 {
() => {
// Module: crate
// Provides: {"impl_109"}
// Dependencies: {}
impl ServerSideStepper < '_ > { fn make_config (params : & BenchmarkParams , resume : ResumptionKind) -> Arc < ServerConfig > { let cfg = ServerConfig :: builder (params . provider . clone ()) ; let mut cfg = match params . auth_key { AuthKeySource :: KeyType (key_type) => cfg . with_client_cert_verifier (WebPkiClientVerifier :: no_client_auth ()) . with_single_cert (key_type . identity () , key_type . key ()) . expect ("bad certs/private key?") , AuthKeySource :: FuzzingProvider => cfg . with_client_cert_verifier (WebPkiClientVerifier :: no_client_auth ()) . with_server_credential_resolver (rustls_fuzzing_provider :: server_cert_resolver ()) . unwrap () , } ; if resume == ResumptionKind :: SessionId { cfg . session_storage = ServerSessionMemoryCache :: new (128) ; } else if resume == ResumptionKind :: Tickets { cfg . ticketer = Some ((params . ticketer) ()) ; } else { cfg . session_storage = Arc :: new (NoServerSessionStorage { }) ; } Arc :: new (cfg) } }
};
}
