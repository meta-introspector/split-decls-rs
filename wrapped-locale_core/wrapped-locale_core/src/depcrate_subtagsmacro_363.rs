// Generated macro for macro_363 (macro)
macro_rules! Depcrate_subtagsmacro_363 {
() => {
// Module: crate::subtags
// Provides: {"macro_363"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A generic subtag."] # [doc = ""] # [doc = " The subtag has to be an ASCII alphanumerical string no shorter than"] # [doc = " two characters and no longer than eight."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::Subtag;"] # [doc = ""] # [doc = " let subtag1: Subtag = \"Foo\".parse()"] # [doc = "     .expect(\"Failed to parse a Subtag.\");"] # [doc = ""] # [doc = " assert_eq!(subtag1.as_str(), \"foo\");"] # [doc = " ```"] Subtag , subtags , subtag , subtags_subtag , 2 ..= 8 , s , s . is_ascii_alphanumeric () , s . to_ascii_lowercase () , s . is_ascii_alphanumeric () && s . is_ascii_lowercase () , InvalidSubtag , ["foo12"] , ["f" , "toolooong"] ,) ;
};
}
