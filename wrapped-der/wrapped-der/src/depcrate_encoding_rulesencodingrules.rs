// Generated macro for EncodingRules (enum)
macro_rules! Depcrate_encoding_rulesEncodingRules {
() => {
// Module: crate::encoding_rules
// Provides: {"EncodingRules"}
// Dependencies: {}
# [doc = " ASN.1 encoding rules."] # [doc = ""] # [doc = " This enum identifies the specific encoding rules which are applied at the time a given document"] # [doc = " is decoded from a byte/octet serialization."] # [doc = ""] # [doc = " In addition to the Distinguished Encoding Rules (DER), this crate also supports a strict subset"] # [doc = " of the Basic Encoding Rules (BER) which supports the minimum amount of additional productions"] # [doc = " beyond DER needed to interoperate with other implementations of cryptography-oriented formats"] # [doc = " which utilize BER, e.g. CMS, PKCS#8."] # [derive (Clone , Copy , Debug , Default , Eq , PartialEq , PartialOrd , Ord)] pub enum EncodingRules { # [doc = " Basic Encoding Rules."] # [cfg (feature = "ber")] Ber , # [doc = " Distinguished Encoding Rules."] # [default] Der , }
};
}
