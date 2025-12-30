// Generated macro for SM4 (static)
macro_rules! Depcrate_typesSM4 {
() => {
// Module: crate::types
// Provides: {"SM4"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_OSSLCONF = "OPENSSL_NO_SM4"))] pub static SM4 : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.primitives.ciphers.algorithms" , & ["SM4"] ,) ;
};
}
