// Generated macro for macro_199 (macro)
macro_rules! Depcrate_extensions_transform_keymacro_199 {
() => {
// Module: crate::extensions::transform::key
// Provides: {"macro_199"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A key used in a list of [`Fields`](super::Fields)."] # [doc = ""] # [doc = " The key has to be a two ASCII characters long, with the first"] # [doc = " character being alphabetic, and the second being a number."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::transform::Key;"] # [doc = ""] # [doc = " let key1: Key = \"k0\".parse().expect(\"Failed to parse a Key.\");"] # [doc = ""] # [doc = " assert_eq!(key1.as_str(), \"k0\");"] # [doc = " ```"] Key , extensions :: transform , key , extensions_transform_key , 2 ..= 2 , s , s . all_bytes () [0] . is_ascii_alphabetic () && s . all_bytes () [1] . is_ascii_digit () , s . to_ascii_lowercase () , s . all_bytes () [0] . is_ascii_lowercase () && s . all_bytes () [1] . is_ascii_digit () , InvalidExtension , ["k0"] , ["" , "k" , "0k" , "k12"] ,) ;
};
}
