// Generated macro for FFDHE_PROVIDER (const)
macro_rules! Depcrate_ffdhe_kx_with_opensslFFDHE_PROVIDER {
() => {
// Module: crate::ffdhe_kx_with_openssl
// Provides: {"FFDHE_PROVIDER"}
// Dependencies: {}
const FFDHE_PROVIDER : CryptoProvider = CryptoProvider { tls12_cipher_suites : Cow :: Borrowed (& [& ffdhe :: TLS_DHE_RSA_WITH_AES_128_GCM_SHA256]) , tls13_cipher_suites : Cow :: Borrowed (& [provider :: cipher_suite :: TLS13_AES_128_GCM_SHA256]) , kx_groups : Cow :: Borrowed (& [FFDHE2048_GROUP]) , .. provider :: DEFAULT_PROVIDER } ;
};
}
