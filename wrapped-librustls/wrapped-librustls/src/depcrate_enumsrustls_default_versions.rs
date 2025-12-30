// Generated macro for RUSTLS_DEFAULT_VERSIONS (static)
macro_rules! Depcrate_enumsRUSTLS_DEFAULT_VERSIONS {
() => {
// Module: crate::enums
// Provides: {"RUSTLS_DEFAULT_VERSIONS"}
// Dependencies: {}
# [doc = " Rustls' default list of protocol versions. The length of the array is"] # [doc = " given by `RUSTLS_DEFAULT_VERSIONS_LEN`."] # [no_mangle] pub static RUSTLS_DEFAULT_VERSIONS : [u16 ; 2] = [rustls_tls_version :: Tlsv1_3 as u16 , rustls_tls_version :: Tlsv1_2 as u16 ,] ;
};
}
