// Generated macro for impl_47 (impl)
macro_rules! Depcrate_langidimpl_47 {
() => {
// Module: crate::langid
// Provides: {"impl_47"}
// Dependencies: {}
# [doc = " Convert from an LSR tuple to a [`LanguageIdentifier`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::{"] # [doc = "     langid,"] # [doc = "     subtags::{language, region, script},"] # [doc = "     LanguageIdentifier,"] # [doc = " };"] # [doc = ""] # [doc = " let lang = language!(\"en\");"] # [doc = " let script = script!(\"Latn\");"] # [doc = " let region = region!(\"US\");"] # [doc = " assert_eq!("] # [doc = "     LanguageIdentifier::from((lang, Some(script), Some(region))),"] # [doc = "     langid!(\"en-Latn-US\")"] # [doc = " );"] # [doc = " ```"] impl From < (subtags :: Language , Option < subtags :: Script > , Option < subtags :: Region > ,) > for LanguageIdentifier { fn from (lsr : (subtags :: Language , Option < subtags :: Script > , Option < subtags :: Region > ,) ,) -> Self { Self { language : lsr . 0 , script : lsr . 1 , region : lsr . 2 , variants : subtags :: Variants :: new () , } } }
};
}
