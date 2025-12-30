// Generated macro for macro_341 (macro)
macro_rules! Depcrate_subtags_regionmacro_341 {
() => {
// Module: crate::subtags::region
// Provides: {"macro_341"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A region subtag (examples: `\"US\"`, `\"CN\"`, `\"AR\"` etc.)"] # [doc = ""] # [doc = " [`Region`] represents a Unicode base language code conformant to the"] # [doc = " [`unicode_region_id`] field of the Language and Locale Identifier."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::subtags::Region;"] # [doc = ""] # [doc = " let region: Region ="] # [doc = "     \"DE\".parse().expect(\"Failed to parse a region subtag.\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`unicode_region_id`]: https://unicode.org/reports/tr35/#unicode_region_id"] Region , subtags , region , subtags_region , 2 ..= 3 , s , if s . len () == 2 { s . is_ascii_alphabetic () } else { s . is_ascii_numeric () } , if s . len () == 2 { s . to_ascii_uppercase () } else { s } , if s . len () == 2 { s . is_ascii_alphabetic_uppercase () } else { s . is_ascii_numeric () } , InvalidSubtag , ["FR" , "123"] , ["12" , "FRA" , "b2"] ,) ;
};
}
