// Generated macro for IDEA (static)
macro_rules! Depcrate_typesIDEA {
() => {
// Module: crate::types
// Provides: {"IDEA"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_OSSLCONF = "OPENSSL_NO_IDEA"))] pub static IDEA : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.decrepit.ciphers.algorithms" , & ["IDEA"]) ;
};
}
