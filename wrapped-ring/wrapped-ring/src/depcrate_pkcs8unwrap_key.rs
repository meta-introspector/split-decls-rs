// Generated macro for unwrap_key (function)
macro_rules! Depcrate_pkcs8unwrap_key {
() => {
// Module: crate::pkcs8
// Provides: {"unwrap_key"}
// Dependencies: {}
# [doc = " Parses an unencrypted PKCS#8 private key, verifies that it is the right type"] # [doc = " of key, and returns the key value."] # [doc = ""] # [doc = " PKCS#8 is specified in [RFC 5958]."] # [doc = ""] # [doc = " [RFC 5958]: https://tools.ietf.org/html/rfc5958"] pub (crate) fn unwrap_key < 'a > (template : & Template , version : Version , input : untrusted :: Input < 'a > ,) -> Result < (untrusted :: Input < 'a > , Option < untrusted :: Input < 'a > >) , error :: KeyRejected > { unwrap_key_ (template . alg_id_value () , version , input) }
};
}
