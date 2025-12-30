// Generated macro for AsKebabCase (struct)
macro_rules! Depcrate_kebabAsKebabCase {
() => {
// Module: crate::kebab
// Provides: {"AsKebabCase"}
// Dependencies: {}
# [doc = " This wrapper performs a kebab case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsKebabCase;"] # [doc = ""] # [doc = " let sentence = \"We are going to inherit the earth.\";"] # [doc = " assert_eq!(format!(\"{}\", AsKebabCase(sentence)), \"we-are-going-to-inherit-the-earth\");"] # [doc = " ```"] pub struct AsKebabCase < T : AsRef < str > > (pub T) ;
};
}
