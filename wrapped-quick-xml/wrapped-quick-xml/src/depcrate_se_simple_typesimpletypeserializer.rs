// Generated macro for SimpleTypeSerializer (struct)
macro_rules! Depcrate_se_simple_typeSimpleTypeSerializer {
() => {
// Module: crate::se::simple_type
// Provides: {"SimpleTypeSerializer"}
// Dependencies: {}
# [doc = " A serializer for a values representing XSD [simple types], which used in:"] # [doc = " - attribute values (`<... ...=\"value\" ...>`)"] # [doc = " - text content (`<...>text</...>`)"] # [doc = " - CDATA content (`<...><![CDATA[cdata]]></...>`)"] # [doc = ""] # [doc = " [simple types]: https://www.w3.org/TR/xmlschema11-1/#Simple_Type_Definition"] pub struct SimpleTypeSerializer < W : Write > { # [doc = " Writer to which this serializer writes content"] pub writer : W , # [doc = " Target for which element is serializing. Affects additional characters to escape."] pub target : QuoteTarget , # [doc = " Defines which XML characters need to be escaped"] pub level : QuoteLevel , }
};
}
