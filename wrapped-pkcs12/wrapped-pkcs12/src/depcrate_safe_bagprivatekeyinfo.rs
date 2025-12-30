// Generated macro for PrivateKeyInfo (struct)
macro_rules! Depcrate_safe_bagPrivateKeyInfo {
() => {
// Module: crate::safe_bag
// Provides: {"PrivateKeyInfo"}
// Dependencies: {}
# [doc = " The `PrivateKeyInfo` type is defined in [RFC 5208 Section 5]."] # [doc = ""] # [doc = " ```text"] # [doc = "       PrivateKeyInfo ::= SEQUENCE {"] # [doc = "         version                   Version,"] # [doc = "         privateKeyAlgorithm       PrivateKeyAlgorithmIdentifier,"] # [doc = "         privateKey                PrivateKey,"] # [doc = "         attributes           [0]  IMPLICIT Attributes OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5208 Section 5]: https://www.rfc-editor.org/rfc/rfc5208#section-5"] # [derive (Clone , Debug , PartialEq , Eq , Sequence)] pub struct PrivateKeyInfo { # [doc = " Syntax version number (always 0 for RFC 5208)"] pub version : Pkcs8Version , # [doc = " X.509 `AlgorithmIdentifier` for the private key type."] pub algorithm : AlgorithmIdentifierOwned , # [doc = " Private key data."] pub private_key : OctetString , # [doc = " Public key data, optionally available if version is V2."] # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub attributes : Option < Attributes > , }
};
}
