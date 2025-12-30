// Generated macro for AsUpperCamelCase (struct)
macro_rules! Depcrate_upper_camelAsUpperCamelCase {
() => {
// Module: crate::upper_camel
// Provides: {"AsUpperCamelCase"}
// Dependencies: {}
# [doc = " This wrapper performs a upper camel case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsUpperCamelCase;"] # [doc = ""] # [doc = " let sentence = \"We are not in the least afraid of ruins.\";"] # [doc = " assert_eq!(format!(\"{}\", AsUpperCamelCase(sentence)), \"WeAreNotInTheLeastAfraidOfRuins\");"] # [doc = " ```"] pub struct AsUpperCamelCase < T : AsRef < str > > (pub T) ;
};
}
