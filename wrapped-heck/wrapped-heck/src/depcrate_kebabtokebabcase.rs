// Generated macro for ToKebabCase (trait)
macro_rules! Depcrate_kebabToKebabCase {
() => {
// Module: crate::kebab
// Provides: {"ToKebabCase"}
// Dependencies: {}
# [doc = " This trait defines a kebab case conversion."] # [doc = ""] # [doc = " In kebab-case, word boundaries are indicated by hyphens."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToKebabCase;"] # [doc = ""] # [doc = " let sentence = \"We are going to inherit the earth.\";"] # [doc = " assert_eq!(sentence.to_kebab_case(), \"we-are-going-to-inherit-the-earth\");"] # [doc = " ```"] pub trait ToKebabCase : ToOwned { # [doc = " Convert this type to kebab case."] fn to_kebab_case (& self) -> Self :: Owned ; }
};
}
