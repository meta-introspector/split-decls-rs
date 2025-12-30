// Generated macro for create_content_type_attribute (function)
macro_rules! Depcrate_buildercreate_content_type_attribute {
() => {
// Module: crate::builder
// Provides: {"create_content_type_attribute"}
// Dependencies: {}
# [doc = " Create a content-type attribute according to"] # [doc = " [RFC 5652 § 11.1](https://datatracker.ietf.org/doc/html/rfc5652#section-11.1)"] pub fn create_content_type_attribute (content_type : ObjectIdentifier) -> Result < Attribute > { let content_type_attribute_value = AttributeValue :: new (Tag :: ObjectIdentifier , content_type . as_bytes ()) ? ; let mut values = SetOfVec :: new () ; values . insert (content_type_attribute_value) ? ; let attribute = Attribute { oid : const_oid :: db :: rfc5911 :: ID_CONTENT_TYPE , values , } ; Ok (attribute) }
};
}
