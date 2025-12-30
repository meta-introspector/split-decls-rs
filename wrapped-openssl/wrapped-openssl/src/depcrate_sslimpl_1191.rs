// Generated macro for impl_1191 (impl)
macro_rules! Depcrate_sslimpl_1191 {
() => {
// Module: crate::ssl
// Provides: {"impl_1191"}
// Dependencies: {}
impl SslVersion { # [doc = " SSLv3"] pub const SSL3 : SslVersion = SslVersion (ffi :: SSL3_VERSION) ; # [doc = " TLSv1.0"] pub const TLS1 : SslVersion = SslVersion (ffi :: TLS1_VERSION) ; # [doc = " TLSv1.1"] pub const TLS1_1 : SslVersion = SslVersion (ffi :: TLS1_1_VERSION) ; # [doc = " TLSv1.2"] pub const TLS1_2 : SslVersion = SslVersion (ffi :: TLS1_2_VERSION) ; # [doc = " TLSv1.3"] # [doc = ""] # [doc = " Requires AWS-LC or BoringSSL or OpenSSL 1.1.1 or newer or LibreSSL."] # [cfg (any (ossl111 , libressl , boringssl , awslc))] pub const TLS1_3 : SslVersion = SslVersion (ffi :: TLS1_3_VERSION) ; # [doc = " DTLSv1.0"] # [doc = ""] # [doc = " DTLS 1.0 corresponds to TLS 1.1."] pub const DTLS1 : SslVersion = SslVersion (ffi :: DTLS1_VERSION) ; # [doc = " DTLSv1.2"] # [doc = ""] # [doc = " DTLS 1.2 corresponds to TLS 1.2 to harmonize versions. There was never a DTLS 1.1."] pub const DTLS1_2 : SslVersion = SslVersion (ffi :: DTLS1_2_VERSION) ; }
};
}
