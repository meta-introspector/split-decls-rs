// Generated macro for resolve_xml_entity (function)
macro_rules! Depcrate_escaperesolve_xml_entity {
() => {
// Module: crate::escape
// Provides: {"resolve_xml_entity"}
// Dependencies: {}
# [doc = " Resolves predefined XML entities. If specified entity is not a predefined XML"] # [doc = " entity, `None` is returned."] # [doc = ""] # [doc = " The complete list of predefined entities are defined in the [specification]."] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::escape::resolve_xml_entity;"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " assert_eq!(resolve_xml_entity(\"lt\"), Some(\"<\"));"] # [doc = " assert_eq!(resolve_xml_entity(\"gt\"), Some(\">\"));"] # [doc = " assert_eq!(resolve_xml_entity(\"amp\"), Some(\"&\"));"] # [doc = " assert_eq!(resolve_xml_entity(\"apos\"), Some(\"'\"));"] # [doc = " assert_eq!(resolve_xml_entity(\"quot\"), Some(\"\\\"\"));"] # [doc = ""] # [doc = " assert_eq!(resolve_xml_entity(\"foo\"), None);"] # [doc = " ```"] # [doc = ""] # [doc = " [specification]: https://www.w3.org/TR/xml11/#sec-predefined-ent"] pub const fn resolve_xml_entity (entity : & str) -> Option < & 'static str > { let s = match entity . as_bytes () { b"lt" => "<" , b"gt" => ">" , b"amp" => "&" , b"apos" => "'" , b"quot" => "\"" , _ => return None , } ; Some (s) }
};
}
