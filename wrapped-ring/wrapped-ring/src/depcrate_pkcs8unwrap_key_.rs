// Generated macro for unwrap_key_ (function)
macro_rules! Depcrate_pkcs8unwrap_key_ {
() => {
// Module: crate::pkcs8
// Provides: {"unwrap_key_"}
// Dependencies: {}
# [doc = " Parses an unencrypted PKCS#8 private key, verifies that it is the right type"] # [doc = " of key, and returns the key value."] # [doc = ""] # [doc = " `alg_id` must be the encoded value (not including the outermost `SEQUENCE`"] # [doc = " tag and length) of the `AlgorithmIdentifier` that identifies the key type."] # [doc = " The result will be an encoded `RSAPrivateKey` or `ECPrivateKey` or similar."] # [doc = ""] # [doc = " PKCS#8 is specified in [RFC 5958]."] # [doc = ""] # [doc = " [RFC 5958]: https://tools.ietf.org/html/rfc5958"] pub (crate) fn unwrap_key_ < 'a > (alg_id : untrusted :: Input , version : Version , input : untrusted :: Input < 'a > ,) -> Result < (untrusted :: Input < 'a > , Option < untrusted :: Input < 'a > >) , error :: KeyRejected > { input . read_all (error :: KeyRejected :: invalid_encoding () , | input | { der :: nested (input , der :: Tag :: Sequence , error :: KeyRejected :: invalid_encoding () , | input | unwrap_key__ (alg_id , version , input) ,) }) }
};
}
