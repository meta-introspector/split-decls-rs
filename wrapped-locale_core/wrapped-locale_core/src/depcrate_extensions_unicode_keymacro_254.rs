// Generated macro for macro_254 (macro)
macro_rules! Depcrate_extensions_unicode_keymacro_254 {
() => {
// Module: crate::extensions::unicode::key
// Provides: {"macro_254"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A key used in a list of [`Keywords`](super::Keywords)."] # [doc = ""] # [doc = " The key has to be a two ASCII alphanumerical characters long, with the first"] # [doc = " character being alphanumeric, and the second being alphabetic."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::unicode::Key;"] # [doc = ""] # [doc = " assert!(\"ca\".parse::<Key>().is_ok());"] # [doc = " ```"] Key , extensions :: unicode , key , extensions_unicode_key , 2 ..= 2 , s , s . all_bytes () [0] . is_ascii_alphanumeric () && s . all_bytes () [1] . is_ascii_alphabetic () , s . to_ascii_lowercase () , (s . all_bytes () [0] . is_ascii_lowercase () || s . all_bytes () [0] . is_ascii_digit ()) && s . all_bytes () [1] . is_ascii_lowercase () , InvalidExtension , ["ca" , "8a"] , ["a" , "a8" , "abc"] ,) ;
};
}
