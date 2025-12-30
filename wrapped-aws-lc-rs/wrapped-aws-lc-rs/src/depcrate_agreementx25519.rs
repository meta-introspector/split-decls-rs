// Generated macro for X25519 (const)
macro_rules! Depcrate_agreementX25519 {
() => {
// Module: crate::agreement
// Provides: {"X25519"}
// Dependencies: {}
# [doc = " X25519 (ECDH using Curve25519) as described in [RFC 7748]."] # [doc = ""] # [doc = " Everything is as described in RFC 7748. Key agreement will fail if the"] # [doc = " result of the X25519 operation is zero; see the notes on the"] # [doc = " \"all-zero value\" in [RFC 7748 section 6.1]."] # [doc = ""] # [doc = " [RFC 7748]: https://tools.ietf.org/html/rfc7748"] # [doc = " [RFC 7748 section 6.1]: https://tools.ietf.org/html/rfc7748#section-6.1"] pub const X25519 : Algorithm = Algorithm { id : AlgorithmID :: X25519 , } ;
};
}
