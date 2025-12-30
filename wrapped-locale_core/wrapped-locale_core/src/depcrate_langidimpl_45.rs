// Generated macro for impl_45 (impl)
macro_rules! Depcrate_langidimpl_45 {
() => {
// Module: crate::langid
// Provides: {"impl_45"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::{langid, subtags::script, LanguageIdentifier};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     LanguageIdentifier::from(Some(script!(\"latn\"))),"] # [doc = "     langid!(\"und-Latn\")"] # [doc = " );"] # [doc = " ```"] impl From < Option < subtags :: Script > > for LanguageIdentifier { fn from (script : Option < subtags :: Script >) -> Self { Self { language : subtags :: Language :: UNKNOWN , script , region : None , variants : subtags :: Variants :: new () , } } }
};
}
