// Generated macro for MacData (struct)
macro_rules! Depcrate_mac_dataMacData {
() => {
// Module: crate::mac_data
// Provides: {"MacData"}
// Dependencies: {}
# [doc = " The `MacData` type is defined in [RFC 7292 Section 4]."] # [doc = ""] # [doc = " ```text"] # [doc = " MacData ::= SEQUENCE {"] # [doc = "     mac         DigestInfo,"] # [doc = "     macSalt     OCTET STRING,"] # [doc = "     iterations  INTEGER DEFAULT 1"] # [doc = "     -- Note: The default is for historical reasons and its"] # [doc = "     --       use is deprecated."] # [doc = "}"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 7292 Section 4]: https://www.rfc-editor.org/rfc/rfc7292#section-4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] pub struct MacData { # [doc = " the MAC digest info"] pub mac : DigestInfo , # [doc = " the MAC salt"] pub mac_salt : OctetString , # [doc = " the number of iterations"] # [asn1 (default = "default_one")] pub iterations : i32 , }
};
}
