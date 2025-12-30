// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " ASN.1 DER-related errors."] Asn1 (der :: Error) , # [doc = " Cryptographic errors."] # [doc = ""] # [doc = " These can be used by RSA implementations to signal that a key is"] # [doc = " invalid for cryptographic reasons. This means the document parsed"] # [doc = " correctly, but one of the values contained within was invalid, e.g."] # [doc = " a number expected to be a prime was not a prime."] Crypto , # [doc = " Malformed cryptographic key contained in a PKCS#1 document."] # [doc = ""] # [doc = " This is intended for relaying errors when decoding fields of a PKCS#1 document."] KeyMalformed , # [doc = " Version errors"] Version , }
};
}
