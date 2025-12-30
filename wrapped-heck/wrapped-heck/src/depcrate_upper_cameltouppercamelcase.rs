// Generated macro for ToUpperCamelCase (trait)
macro_rules! Depcrate_upper_camelToUpperCamelCase {
() => {
// Module: crate::upper_camel
// Provides: {"ToUpperCamelCase"}
// Dependencies: {}
# [doc = " This trait defines an upper camel case conversion."] # [doc = ""] # [doc = " In UpperCamelCase, word boundaries are indicated by capital letters,"] # [doc = " including the first word."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToUpperCamelCase;"] # [doc = ""] # [doc = " let sentence = \"We are not in the least afraid of ruins.\";"] # [doc = " assert_eq!(sentence.to_upper_camel_case(), \"WeAreNotInTheLeastAfraidOfRuins\");"] # [doc = " ```"] pub trait ToUpperCamelCase : ToOwned { # [doc = " Convert this type to upper camel case."] fn to_upper_camel_case (& self) -> Self :: Owned ; }
};
}
