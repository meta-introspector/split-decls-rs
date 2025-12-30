// Generated macro for SslVersion (enum)
macro_rules! Depcrate_easy_handlerSslVersion {
() => {
// Module: crate::easy::handler
// Provides: {"SslVersion"}
// Dependencies: {}
# [doc = " Possible values to pass to the `ssl_version` and `ssl_min_max_version` method."] # [non_exhaustive] # [allow (missing_docs)] # [derive (Debug , Clone , Copy)] pub enum SslVersion { Default = curl_sys :: CURL_SSLVERSION_DEFAULT as isize , Tlsv1 = curl_sys :: CURL_SSLVERSION_TLSv1 as isize , Sslv2 = curl_sys :: CURL_SSLVERSION_SSLv2 as isize , Sslv3 = curl_sys :: CURL_SSLVERSION_SSLv3 as isize , Tlsv10 = curl_sys :: CURL_SSLVERSION_TLSv1_0 as isize , Tlsv11 = curl_sys :: CURL_SSLVERSION_TLSv1_1 as isize , Tlsv12 = curl_sys :: CURL_SSLVERSION_TLSv1_2 as isize , Tlsv13 = curl_sys :: CURL_SSLVERSION_TLSv1_3 as isize , }
};
}
