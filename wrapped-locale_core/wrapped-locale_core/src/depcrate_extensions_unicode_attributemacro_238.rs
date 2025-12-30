// Generated macro for macro_238 (macro)
macro_rules! Depcrate_extensions_unicode_attributemacro_238 {
() => {
// Module: crate::extensions::unicode::attribute
// Provides: {"macro_238"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " An attribute used in a set of [`Attributes`](super::Attributes)."] # [doc = ""] # [doc = " An attribute has to be a sequence of alphanumerical characters no"] # [doc = " shorter than three and no longer than eight characters."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::unicode::{attribute, Attribute};"] # [doc = ""] # [doc = " let attr: Attribute ="] # [doc = "     \"buddhist\".parse().expect(\"Failed to parse an Attribute.\");"] # [doc = ""] # [doc = " assert_eq!(attr, attribute!(\"buddhist\"));"] # [doc = " ```"] Attribute , extensions :: unicode , attribute , extensions_unicode_attribute , 3 ..= 8 , s , s . is_ascii_alphanumeric () , s . to_ascii_lowercase () , s . is_ascii_alphanumeric () && s . is_ascii_lowercase () , InvalidExtension , ["foo12"] , ["no" , "toolooong"] ,) ;
};
}
