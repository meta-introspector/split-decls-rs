// Generated macro for FromAttributes (trait)
macro_rules! Depcrate_from_attributesFromAttributes {
() => {
// Module: crate::from_attributes
// Provides: {"FromAttributes"}
// Dependencies: {}
# [doc = " Create an instance by parsing a list of attributes."] # [doc = ""] # [doc = " This trait is useful when dealing with items such as traits on traits and impl blocks,"] # [doc = " for which `darling` does not provide dedicated traits."] pub trait FromAttributes : Sized { # [doc = " Create an instance by parsing a list of attributes."] # [doc = ""] # [doc = " By convention, `FromAttributes` implementations should merge item"] # [doc = " declarations across attributes, so that the following forms are"] # [doc = " equivalent:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[derive(Serialize)]"] # [doc = " #[serde(rename_all = \"camel_case\")]"] # [doc = " #[serde(borrow)]"] # [doc = " pub struct SplitExample {}"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " #[serde(borrow, rename_all = \"camel_case\")]"] # [doc = " pub struct JoinedExample {}"] # [doc = " ```"] fn from_attributes (attrs : & [Attribute]) -> Result < Self > ; }
};
}
