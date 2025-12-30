// Generated macro for EnumValue (struct)
macro_rules! Depcrate_schema_metaEnumValue {
() => {
// Module: crate::schema::meta
// Provides: {"EnumValue"}
// Dependencies: {}
# [doc = " Metadata for a single value in an enum"] # [derive (Debug , Clone)] pub struct EnumValue { # [doc = " The name of the enum value"] # [doc = ""] # [doc = " This is the string literal representation of the enum in responses."] pub name : ArcStr , # [doc = " The optional description of the enum value."] # [doc = ""] # [doc = " Note: this is not the description of the enum itself; it's the"] # [doc = " description of this enum _value_."] pub description : Option < ArcStr > , # [doc = " Whether the field is deprecated or not, with an optional reason."] pub deprecation_status : DeprecationStatus , }
};
}
