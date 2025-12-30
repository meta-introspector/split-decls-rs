// Generated macro for cms (module)
macro_rules! Depcratecms {
() => {
// Module: crate
// Provides: {"cms"}
// Dependencies: {}
# [cfg (all (not (libressl) , not (osslconf = "OPENSSL_NO_CMS")))] pub mod cms ;
};
}
