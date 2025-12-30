// Generated macro for ListFormatterOptions (struct)
macro_rules! Depcrate_optionsListFormatterOptions {
() => {
// Module: crate::options
// Provides: {"ListFormatterOptions"}
// Dependencies: {}
# [doc = " A list of options set by the developer to adjust the behavior of the ListFormatter."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use icu::list::options::{ListFormatterOptions, ListLength};"] # [doc = ""] # [doc = " let options = ListFormatterOptions::default().with_length(ListLength::Wide);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub struct ListFormatterOptions { # [doc = " The length variant should reflect available space for the list."] pub length : Option < ListLength > , }
};
}
