// Generated macro for ARC4 (static)
macro_rules! Depcrate_typesARC4 {
() => {
// Module: crate::types
// Provides: {"ARC4"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_OSSLCONF = "OPENSSL_NO_RC4"))] pub static ARC4 : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.decrepit.ciphers.algorithms" , & ["ARC4"]) ;
};
}
