// Generated macro for macro_338 (macro)
macro_rules! Depcrate_subtags_languagemacro_338 {
() => {
// Module: crate::subtags::language
// Provides: {"macro_338"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A language subtag (examples: `\"en\"`, `\"csb\"`, `\"zh\"`, `\"und\"`, etc.)"] # [doc = ""] # [doc = " [`Language`] represents a Unicode base language code conformant to the"] # [doc = " [`unicode_language_id`] field of the Language and Locale Identifier."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::Language;"] # [doc = ""] # [doc = " let language: Language ="] # [doc = "     \"en\".parse().expect(\"Failed to parse a language subtag.\");"] # [doc = " ```"] # [doc = ""] # [doc = " If the [`Language`] has no value assigned, it serializes to a string `\"und\"`, which"] # [doc = " can be then parsed back to an empty [`Language`] field."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::Language;"] # [doc = ""] # [doc = " assert_eq!(Language::UNKNOWN.as_str(), \"und\");"] # [doc = " ```"] # [doc = ""] # [doc = " `Notice`: ICU4X uses a narrow form of language subtag of 2-3 characters."] # [doc = " The specification allows language subtag to optionally also be 5-8 characters"] # [doc = " but that form has not been used and ICU4X does not support it right now."] # [doc = ""] # [doc = " [`unicode_language_id`]: https://unicode.org/reports/tr35/#unicode_language_id"] Language , subtags , language , subtags_language , 2 ..= 3 , s , s . is_ascii_alphabetic () , s . to_ascii_lowercase () , s . is_ascii_alphabetic_lowercase () , InvalidLanguage , ["en" , "foo"] , ["419" , "german" , "en1"] ,) ;
};
}
