// Generated macro for TrailingCase (enum)
macro_rules! Depcrate_titlecaseTrailingCase {
() => {
// Module: crate::titlecase
// Provides: {"TrailingCase"}
// Dependencies: {}
# [doc = " How to handle the rest of the string once the beginning of the"] # [doc = " string has been titlecased."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::casemap::options::{TitlecaseOptions, TrailingCase};"] # [doc = " use icu::casemap::TitlecaseMapper;"] # [doc = " use icu::locale::langid;"] # [doc = ""] # [doc = " let cm = TitlecaseMapper::new();"] # [doc = " let root = langid!(\"und\");"] # [doc = ""] # [doc = " let default_options = Default::default();"] # [doc = " let mut preserve_case: TitlecaseOptions = Default::default();"] # [doc = " preserve_case.trailing_case = Some(TrailingCase::Unchanged);"] # [doc = ""] # [doc = " // Exhibits trailing case when set:"] # [doc = " assert_eq!("] # [doc = "     cm.titlecase_segment_to_string(\"spOngeBoB\", &root, default_options),"] # [doc = "     \"Spongebob\""] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     cm.titlecase_segment_to_string(\"spOngeBoB\", &root, preserve_case),"] # [doc = "     \"SpOngeBoB\""] # [doc = " );"] # [doc = " ```"] # [non_exhaustive] # [derive (Copy , Clone , Default , PartialEq , Eq , Hash , Debug)] pub enum TrailingCase { # [doc = " Preserve the casing of the rest of the string (\"spoNgEBoB\" -> \"SpoNgEBoB\")"] Unchanged , # [doc = " Lowercase the rest of the string (\"spoNgEBoB\" -> \"Spongebob\")"] # [default] Lower , }
};
}
