// Generated macro for macro_348 (macro)
macro_rules! Depcrate_subtags_variantmacro_348 {
() => {
// Module: crate::subtags::variant
// Provides: {"macro_348"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A variant subtag (examples: `\"macos\"`, `\"posix\"`, `\"1996\"` etc.)"] # [doc = ""] # [doc = " [`Variant`] represents a Unicode base language code conformant to the"] # [doc = " [`unicode_variant_id`] field of the Language and Locale Identifier."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::Variant;"] # [doc = ""] # [doc = " let variant: Variant ="] # [doc = "     \"macos\".parse().expect(\"Failed to parse a variant subtag.\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`unicode_variant_id`]: https://unicode.org/reports/tr35/#unicode_variant_id"] Variant , subtags , variant , subtags_variant , 4 ..= 8 , s , s . is_ascii_alphanumeric () && (s . len () != 4 || s . all_bytes () [0] . is_ascii_digit ()) , s . to_ascii_lowercase () , s . is_ascii_lowercase () && s . is_ascii_alphanumeric () && (s . len () != 4 || s . all_bytes () [0] . is_ascii_digit ()) , InvalidSubtag , ["posix" , "1996"] , ["yes"] ,) ;
};
}
