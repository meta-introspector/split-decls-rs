// Generated macro for BLOWFISH (static)
macro_rules! Depcrate_typesBLOWFISH {
() => {
// Module: crate::types
// Provides: {"BLOWFISH"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_OSSLCONF = "OPENSSL_NO_BF"))] pub static BLOWFISH : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.decrepit.ciphers.algorithms" , & ["Blowfish"] ,) ;
};
}
