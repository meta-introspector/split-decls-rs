// Generated macro for extensions_unicode_value (macro)
macro_rules! Depcrate_extensions_unicode_valueextensions_unicode_value {
() => {
// Module: crate::extensions::unicode::value
// Provides: {"extensions_unicode_value"}
// Dependencies: {}
# [doc = " A macro allowing for compile-time construction of valid Unicode [`Value`] subtag."] # [doc = ""] # [doc = " The macro only supports single-subtag values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::unicode::{key, value};"] # [doc = " use icu::locale::Locale;"] # [doc = ""] # [doc = " let loc: Locale = \"de-u-ca-buddhist\".parse().unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     loc.extensions.unicode.keywords.get(&key!(\"ca\")),"] # [doc = "     Some(&value!(\"buddhist\"))"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " [`Value`]: crate::extensions::unicode::Value"] # [macro_export] # [doc (hidden)] macro_rules ! extensions_unicode_value { ($ value : literal) => { const { $ crate :: extensions :: unicode :: Value :: from_subtag (match $ crate :: subtags :: Subtag :: try_from_utf8 ($ value . as_bytes ()) { Ok (r) => Some (r) , _ => panic ! (concat ! ("Invalid Unicode extension value: " , $ value)) , } ,) } } ; }
};
}
