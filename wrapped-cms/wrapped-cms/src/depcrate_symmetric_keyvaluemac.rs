// Generated macro for ValueMac (struct)
macro_rules! Depcrate_symmetric_keyValueMac {
() => {
// Module: crate::symmetric_key
// Provides: {"ValueMac"}
// Dependencies: {}
# [doc = " The `ValueMac` type is defined in [RFC 6031 Section 3.2.12]."] # [doc = ""] # [doc = " ```text"] # [doc = "    ValueMac ::= SEQUENCE {"] # [doc = "      macAlgorithm UTF8String,"] # [doc = "      mac          UTF8String }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.2.12]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.2.12"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct ValueMac { pub mac_algorithm : String , pub mac : String , }
};
}
