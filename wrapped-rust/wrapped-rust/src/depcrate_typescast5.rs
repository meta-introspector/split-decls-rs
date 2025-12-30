// Generated macro for CAST5 (static)
macro_rules! Depcrate_typesCAST5 {
() => {
// Module: crate::types
// Provides: {"CAST5"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_OSSLCONF = "OPENSSL_NO_CAST"))] pub static CAST5 : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.decrepit.ciphers.algorithms" , & ["CAST5"] ,) ;
};
}
