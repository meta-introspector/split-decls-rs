// Generated macro for macro_1240 (macro)
macro_rules! Depcrate_sslmacro_1240 {
() => {
// Module: crate::ssl
// Provides: {"macro_1240"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: { TLS_method , DTLS_method , TLS_client_method , TLS_server_method , DTLS_server_method , DTLS_client_method } ; } else { use ffi :: { SSLv23_method as TLS_method , DTLSv1_method as DTLS_method , SSLv23_client_method as TLS_client_method , SSLv23_server_method as TLS_server_method , } ; } }
};
}
