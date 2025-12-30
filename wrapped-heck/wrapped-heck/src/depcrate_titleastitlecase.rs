// Generated macro for AsTitleCase (struct)
macro_rules! Depcrate_titleAsTitleCase {
() => {
// Module: crate::title
// Provides: {"AsTitleCase"}
// Dependencies: {}
# [doc = " This wrapper performs a title case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsTitleCase;"] # [doc = ""] # [doc = " let sentence = \"We have always lived in slums and holes in the wall.\";"] # [doc = " assert_eq!(format!(\"{}\", AsTitleCase(sentence)), \"We Have Always Lived In Slums And Holes In The Wall\");"] # [doc = " ```"] pub struct AsTitleCase < T : AsRef < str > > (pub T) ;
};
}
