// Generated macro for check_for_headers (function)
macro_rules! Depcrate_decodercheck_for_headers {
() => {
// Module: crate::decoder
// Provides: {"check_for_headers"}
// Dependencies: {}
# [doc = " Check for PEM headers in the input, as they are disallowed by RFC7468."] # [doc = ""] # [doc = " Returns `Error::HeaderDisallowed` if headers are encountered."] fn check_for_headers (pem : & [u8] , err : Error) -> Error { if err == Error :: Base64 (base64ct :: Error :: InvalidEncoding) && pem . contains (& grammar :: CHAR_COLON) { Error :: HeaderDisallowed } else { err } }
};
}
