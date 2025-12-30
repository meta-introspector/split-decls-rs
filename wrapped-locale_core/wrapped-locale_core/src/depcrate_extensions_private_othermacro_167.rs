// Generated macro for macro_167 (macro)
macro_rules! Depcrate_extensions_private_othermacro_167 {
() => {
// Module: crate::extensions::private::other
// Provides: {"macro_167"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A single item used in a list of [`Private`](super::Private) extensions."] # [doc = ""] # [doc = " The subtag has to be an ASCII alphanumerical string no shorter than"] # [doc = " one character and no longer than eight."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::private::Subtag;"] # [doc = ""] # [doc = " let subtag1: Subtag = \"Foo\".parse()"] # [doc = "     .expect(\"Failed to parse a Subtag.\");"] # [doc = ""] # [doc = " assert_eq!(subtag1.as_str(), \"foo\");"] # [doc = " ```"] # [doc = ""] # [doc = " Notice: This is different from the generic [`Subtag`](crate::subtags::Subtag)"] # [doc = " which is between two and eight characters."] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::private;"] # [doc = " use icu::locale::subtags;"] # [doc = ""] # [doc = " let subtag: Result<private::Subtag, _> = \"f\".parse();"] # [doc = " assert!(subtag.is_ok());"] # [doc = ""] # [doc = " let subtag: Result<subtags::Subtag, _> = \"f\".parse();"] # [doc = " assert!(subtag.is_err());"] # [doc = " ```"] Subtag , extensions :: private , subtag , extensions_private_subtag , 1 ..= 8 , s , s . is_ascii_alphanumeric () , s . to_ascii_lowercase () , s . is_ascii_alphanumeric () && s . is_ascii_lowercase () , InvalidExtension , ["foo12"] , ["toolooong"] ,) ;
};
}
