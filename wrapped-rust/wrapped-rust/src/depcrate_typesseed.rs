// Generated macro for SEED (static)
macro_rules! Depcrate_typesSEED {
() => {
// Module: crate::types
// Provides: {"SEED"}
// Dependencies: {}
# [cfg (not (CRYPTOGRAPHY_OSSLCONF = "OPENSSL_NO_SEED"))] pub static SEED : LazyPyImport = LazyPyImport :: new ("cryptography.hazmat.decrepit.ciphers.algorithms" , & ["SEED"]) ;
};
}
