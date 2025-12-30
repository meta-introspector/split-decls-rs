// Generated macro for FieldAttrs (struct)
macro_rules! Depcrate_attributesFieldAttrs {
() => {
// Module: crate::attributes
// Provides: {"FieldAttrs"}
// Dependencies: {}
# [doc = " Field-level attributes."] # [derive (Clone , Debug , Default)] pub (crate) struct FieldAttrs { # [doc = " Value of the `#[asn1(type = \"...\")]` attribute if provided."] pub asn1_type : Option < Asn1Type > , # [doc = " Is the inner type constructed?"] pub constructed : bool , # [doc = " Class and number from the following attributes:"] # [doc = " - `#[asn1(application = \"...\")]`"] # [doc = " - `#[asn1(context_specific = \"...\")]`"] # [doc = " - `#[asn1(private = \"...\")]`"] pub class_num : Option < ClassNum > , # [doc = " Indicates name of function that supplies the default value, which will be used in cases"] # [doc = " where encoding is omitted per DER and to omit the encoding per DER"] pub default : Option < Path > , # [doc = " Should we add `&` before `self.field_name`?"] pub should_deref : bool , # [doc = " Is this field \"extensible\", i.e. preceded by the `...` extensibility marker?"] pub extensible : bool , # [doc = " Is this field `OPTIONAL`?"] pub optional : bool , # [doc = " Tagging mode for this type: `EXPLICIT` or `IMPLICIT`, supplied as"] # [doc = " `#[asn1(tag_mode = \"...\")]`."] # [doc = ""] # [doc = " Inherits from the type-level tagging mode if specified, or otherwise"] # [doc = " defaults to `EXPLICIT`."] pub tag_mode : TagMode , }
};
}
