// Generated macro for ToTitleCase (trait)
macro_rules! Depcrate_titleToTitleCase {
() => {
// Module: crate::title
// Provides: {"ToTitleCase"}
// Dependencies: {}
# [doc = " This trait defines a title case conversion."] # [doc = ""] # [doc = " In Title Case, word boundaries are indicated by spaces, and every word is"] # [doc = " capitalized."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToTitleCase;"] # [doc = ""] # [doc = " let sentence = \"We have always lived in slums and holes in the wall.\";"] # [doc = " assert_eq!(sentence.to_title_case(), \"We Have Always Lived In Slums And Holes In The Wall\");"] # [doc = " ```"] pub trait ToTitleCase : ToOwned { # [doc = " Convert this type to title case."] fn to_title_case (& self) -> Self :: Owned ; }
};
}
