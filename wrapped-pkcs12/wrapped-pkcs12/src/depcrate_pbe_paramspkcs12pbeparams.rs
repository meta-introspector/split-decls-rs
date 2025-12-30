// Generated macro for Pkcs12PbeParams (struct)
macro_rules! Depcrate_pbe_paramsPkcs12PbeParams {
() => {
// Module: crate::pbe_params
// Provides: {"Pkcs12PbeParams"}
// Dependencies: {}
# [doc = " The `pkcs-12PbeParams` type is defined in [RFC 7292 Appendix C]."] # [doc = ""] # [doc = "```text"] # [doc = "    pkcs-12PbeParams ::= SEQUENCE {"] # [doc = "```"] # [doc = ""] # [doc = " [RFC 7292 Appendix C]: https://www.rfc-editor.org/rfc/rfc7292#appendix-C"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] pub struct Pkcs12PbeParams { # [doc = " the MAC digest info"] pub salt : OctetString , # [doc = " the number of iterations"] pub iterations : i32 , }
};
}
