// Generated macro for SafeBag (struct)
macro_rules! Depcrate_safe_bagSafeBag {
() => {
// Module: crate::safe_bag
// Provides: {"SafeBag"}
// Dependencies: {}
# [doc = " The `SafeBag` type is defined in [RFC 7292 Section 4.2]."] # [doc = ""] # [doc = " ```text"] # [doc = " SafeBag ::= SEQUENCE {"] # [doc = "     bagId          BAG-TYPE.&id ({PKCS12BagSet})"] # [doc = "     bagValue       [0] EXPLICIT BAG-TYPE.&Type({PKCS12BagSet}{@bagId}),"] # [doc = "     bagAttributes  SET OF PKCS12Attribute OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 7292 Section 4.2]: https://www.rfc-editor.org/rfc/rfc7292#section-4.2"] # [derive (Clone , Debug , Eq , PartialEq)] # [allow (missing_docs)] pub struct SafeBag { pub bag_id : ObjectIdentifier , pub bag_value : Vec < u8 > , pub bag_attributes : Option < Attributes > , }
};
}
