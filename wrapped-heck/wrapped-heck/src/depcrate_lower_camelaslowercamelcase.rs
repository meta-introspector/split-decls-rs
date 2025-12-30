// Generated macro for AsLowerCamelCase (struct)
macro_rules! Depcrate_lower_camelAsLowerCamelCase {
() => {
// Module: crate::lower_camel
// Provides: {"AsLowerCamelCase"}
// Dependencies: {}
# [doc = " This wrapper performs a lower camel case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsLowerCamelCase;"] # [doc = ""] # [doc = " let sentence = \"It is we who built these palaces and cities.\";"] # [doc = " assert_eq!(format!(\"{}\", AsLowerCamelCase(sentence)), \"itIsWeWhoBuiltThesePalacesAndCities\");"] # [doc = " ```"] pub struct AsLowerCamelCase < T : AsRef < str > > (pub T) ;
};
}
