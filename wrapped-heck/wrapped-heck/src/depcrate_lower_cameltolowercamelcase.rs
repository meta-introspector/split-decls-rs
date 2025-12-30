// Generated macro for ToLowerCamelCase (trait)
macro_rules! Depcrate_lower_camelToLowerCamelCase {
() => {
// Module: crate::lower_camel
// Provides: {"ToLowerCamelCase"}
// Dependencies: {}
# [doc = " This trait defines a lower camel case conversion."] # [doc = ""] # [doc = " In lowerCamelCase, word boundaries are indicated by capital letters,"] # [doc = " excepting the first word."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToLowerCamelCase;"] # [doc = ""] # [doc = " let sentence = \"It is we who built these palaces and cities.\";"] # [doc = " assert_eq!(sentence.to_lower_camel_case(), \"itIsWeWhoBuiltThesePalacesAndCities\");"] # [doc = " ```"] pub trait ToLowerCamelCase : ToOwned { # [doc = " Convert this type to lower camel case."] fn to_lower_camel_case (& self) -> Self :: Owned ; }
};
}
