// Generated macro for FieldConstructor (enum)
macro_rules! Depcrate_field_attributesFieldConstructor {
() => {
// Module: crate::field_attributes
// Provides: {"FieldConstructor"}
// Dependencies: {}
# [doc = " Determines how a value for a field should be constructed."] # [cfg_attr (test , derive (Debug))] pub enum FieldConstructor { # [doc = " Assume that Arbitrary is defined for the type of this field and use it (default)"] Arbitrary , # [doc = " Places `Default::default()` as a field value."] Default , # [doc = " Use custom function or closure to generate a value for a field."] With (TokenStream) , # [doc = " Set a field always to the given value."] Value (TokenStream) , }
};
}
