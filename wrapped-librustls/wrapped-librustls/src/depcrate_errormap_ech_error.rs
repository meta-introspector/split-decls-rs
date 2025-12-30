// Generated macro for map_ech_error (function)
macro_rules! Depcrate_errormap_ech_error {
() => {
// Module: crate::error
// Provides: {"map_ech_error"}
// Dependencies: {}
fn map_ech_error (err : EncryptedClientHelloError) -> rustls_result { use rustls_result :: * ; match err { EncryptedClientHelloError :: InvalidConfigList => { InvalidEncryptedClientHelloInvalidConfigList } EncryptedClientHelloError :: NoCompatibleConfig => { InvalidEncryptedClientHelloNoCompatibleConfig } EncryptedClientHelloError :: SniRequired => InvalidEncryptedClientHelloSniRequired , _ => General , } }
};
}
