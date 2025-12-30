// Generated macro for TypeAttrs (struct)
macro_rules! Depcrate_attributesTypeAttrs {
() => {
// Module: crate::attributes
// Provides: {"TypeAttrs"}
// Dependencies: {}
# [doc = " Attributes on a `struct` or `enum` type."] # [derive (Clone , Debug , Default)] pub (crate) struct TypeAttrs { # [doc = " Tagging mode for this type: `EXPLICIT` or `IMPLICIT`, supplied as"] # [doc = " `#[asn1(tag_mode = \"...\")]`."] # [doc = ""] # [doc = " The default value is `EXPLICIT`."] pub tag_mode : TagMode , pub error : ErrorType , }
};
}
