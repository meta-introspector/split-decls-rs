// Generated macro for parse_attribute (function)
macro_rules! Depcrate_field_attributesparse_attribute {
() => {
// Module: crate::field_attributes
// Provides: {"parse_attribute"}
// Dependencies: {}
fn parse_attribute (attr : & Attribute) -> Result < FieldConstructor > { if let Meta :: List (ref meta_list) = attr . meta { parse_attribute_internals (meta_list) } else { let msg = format ! ("#[{ARBITRARY_ATTRIBUTE_NAME}] must contain a group") ; Err (syn :: Error :: new (attr . span () , msg)) } }
};
}
