// Generated macro for create_message_digest_attribute (function)
macro_rules! Depcrate_buildercreate_message_digest_attribute {
() => {
// Module: crate::builder
// Provides: {"create_message_digest_attribute"}
// Dependencies: {}
# [doc = " Create a message digest attribute according to"] # [doc = " [RFC 5652 § 11.2](https://datatracker.ietf.org/doc/html/rfc5652#section-11.2)"] pub fn create_message_digest_attribute (message_digest : & [u8]) -> Result < Attribute > { let message_digest_der = OctetStringRef :: new (message_digest) ? ; let message_digest_attribute_value = AttributeValue :: new (Tag :: OctetString , message_digest_der . as_bytes ()) ? ; let mut values = SetOfVec :: new () ; values . insert (message_digest_attribute_value) ? ; let attribute = Attribute { oid : const_oid :: db :: rfc5911 :: ID_MESSAGE_DIGEST , values , } ; Ok (attribute) }
};
}
