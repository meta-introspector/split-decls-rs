// Generated macro for AsShoutyKebabCase (struct)
macro_rules! Depcrate_shouty_kebabAsShoutyKebabCase {
() => {
// Module: crate::shouty_kebab
// Provides: {"AsShoutyKebabCase"}
// Dependencies: {}
# [doc = " This wrapper performs a kebab case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsShoutyKebabCase;"] # [doc = ""] # [doc = " let sentence = \"We are going to inherit the earth.\";"] # [doc = " assert_eq!(format!(\"{}\", AsShoutyKebabCase(sentence)), \"WE-ARE-GOING-TO-INHERIT-THE-EARTH\");"] # [doc = " ```"] pub struct AsShoutyKebabCase < T : AsRef < str > > (pub T) ;
};
}
