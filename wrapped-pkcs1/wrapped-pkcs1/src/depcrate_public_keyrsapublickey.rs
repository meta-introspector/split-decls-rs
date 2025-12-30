// Generated macro for RsaPublicKey (struct)
macro_rules! Depcrate_public_keyRsaPublicKey {
() => {
// Module: crate::public_key
// Provides: {"RsaPublicKey"}
// Dependencies: {}
# [doc = " PKCS#1 RSA Public Keys as defined in [RFC 8017 Appendix 1.1]."] # [doc = ""] # [doc = " ASN.1 structure containing a serialized RSA public key:"] # [doc = ""] # [doc = " ```text"] # [doc = " RSAPublicKey ::= SEQUENCE {"] # [doc = "     modulus           INTEGER,  -- n"] # [doc = "     publicExponent    INTEGER   -- e"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8017 Appendix 1.1]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.1.1"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct RsaPublicKey < 'a > { # [doc = " `n`: RSA modulus"] pub modulus : UintRef < 'a > , # [doc = " `e`: RSA public exponent"] pub public_exponent : UintRef < 'a > , }
};
}
