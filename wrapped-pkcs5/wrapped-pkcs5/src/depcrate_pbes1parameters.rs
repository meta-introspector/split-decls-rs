// Generated macro for Parameters (struct)
macro_rules! Depcrate_pbes1Parameters {
() => {
// Module: crate::pbes1
// Provides: {"Parameters"}
// Dependencies: {}
# [doc = " Password-Based Encryption Scheme 1 parameters as defined in [RFC 8018 Appendix A.3]."] # [doc = ""] # [doc = " ```text"] # [doc = " PBEParameter ::= SEQUENCE {"] # [doc = "    salt OCTET STRING (SIZE(8)),"] # [doc = "    iterationCount INTEGER }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 8018 Appendix A.3]: https://tools.ietf.org/html/rfc8018#appendix-A.3"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Parameters { # [doc = " Salt value"] pub salt : [u8 ; SALT_LENGTH] , # [doc = " Iteration count"] pub iteration_count : u16 , }
};
}
