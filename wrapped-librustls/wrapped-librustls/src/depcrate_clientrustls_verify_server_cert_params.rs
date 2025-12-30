// Generated macro for rustls_verify_server_cert_params (struct)
macro_rules! Depcrate_clientrustls_verify_server_cert_params {
() => {
// Module: crate::client
// Provides: {"rustls_verify_server_cert_params"}
// Dependencies: {}
# [doc = " Input to a custom certificate verifier callback."] # [doc = ""] # [doc = " See `rustls_client_config_builder_dangerous_set_certificate_verifier()`."] # [doc = ""] # [doc = " server_name can contain a hostname, an IPv4 address in textual form, or an"] # [doc = " IPv6 address in textual form."] # [allow (non_camel_case_types)] # [repr (C)] pub struct rustls_verify_server_cert_params < 'a > { pub end_entity_cert_der : rustls_slice_bytes < 'a > , pub intermediate_certs_der : & 'a rustls_slice_slice_bytes < 'a > , pub server_name : rustls_str < 'a > , pub ocsp_response : rustls_slice_bytes < 'a > , }
};
}
