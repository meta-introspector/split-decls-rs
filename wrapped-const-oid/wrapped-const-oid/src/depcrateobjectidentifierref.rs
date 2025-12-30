// Generated macro for ObjectIdentifierRef (struct)
macro_rules! DepcrateObjectIdentifierRef {
() => {
// Module: crate
// Provides: {"ObjectIdentifierRef"}
// Dependencies: {}
# [doc = " OID reference type: wrapper for the BER serialization."] # [derive (Eq , Hash , PartialEq , PartialOrd , Ord)] # [repr (transparent)] pub struct ObjectIdentifierRef { # [doc = " BER/DER-serialized bytes (sans ASN.1 tag/length)."] ber : [u8] , }
};
}
