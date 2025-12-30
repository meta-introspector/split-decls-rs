// Generated macro for CAMELLIA (static)
macro_rules! Depcrate_typesCAMELLIA {
() => {
// Module: crate::types
// Provides: {"CAMELLIA"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_OSSLCONF = "OPENSSL_NO_CAMELLIA"))] pub static CAMELLIA : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.decrepit.ciphers.algorithms" , & ["Camellia"] ,) ;
};
}
