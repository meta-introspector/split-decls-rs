// Generated macro for Field (struct)
macro_rules! Depcrate_provider_fieldsField {
() => {
// Module: crate::provider::fields
// Provides: {"Field"}
// Dependencies: {}
# [doc = " A field within a date pattern string, also referred to as a date field."] # [doc = ""] # [doc = " A date field is the"] # [doc = " repetition of a specific pattern character one or more times within the pattern string."] # [doc = " The pattern character is known as the field symbol, which indicates the particular meaning for the field."] # [derive (Debug , Eq , PartialEq , Clone , Copy , Ord , PartialOrd)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: fields))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [zerovec :: make_ule (FieldULE)] pub struct Field { # [doc = " The field symbol for the `Field`, which corresponds to the field's meaning with the"] # [doc = " date pattern."] pub symbol : FieldSymbol , # [doc = " The length of the `Field`, which in conjunction with the `FieldSymbol` informs the width or"] # [doc = " style of the formatting output corresponding to this field."] pub length : FieldLength , }
};
}
