// Generated macro for ToShoutyKebabCase (trait)
macro_rules! Depcrate_shouty_kebabToShoutyKebabCase {
() => {
// Module: crate::shouty_kebab
// Provides: {"ToShoutyKebabCase"}
// Dependencies: {}
# [doc = " This trait defines a shouty kebab case conversion."] # [doc = ""] # [doc = " In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all"] # [doc = " words are in uppercase."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToShoutyKebabCase;"] # [doc = ""] # [doc = " let sentence = \"We are going to inherit the earth.\";"] # [doc = " assert_eq!(sentence.to_shouty_kebab_case(), \"WE-ARE-GOING-TO-INHERIT-THE-EARTH\");"] # [doc = " ```"] pub trait ToShoutyKebabCase : ToOwned { # [doc = " Convert this type to shouty kebab case."] fn to_shouty_kebab_case (& self) -> Self :: Owned ; }
};
}
