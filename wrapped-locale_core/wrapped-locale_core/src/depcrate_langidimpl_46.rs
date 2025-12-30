// Generated macro for impl_46 (impl)
macro_rules! Depcrate_langidimpl_46 {
() => {
// Module: crate::langid
// Provides: {"impl_46"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::{langid, subtags::region, LanguageIdentifier};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     LanguageIdentifier::from(Some(region!(\"US\"))),"] # [doc = "     langid!(\"und-US\")"] # [doc = " );"] # [doc = " ```"] impl From < Option < subtags :: Region > > for LanguageIdentifier { fn from (region : Option < subtags :: Region >) -> Self { Self { language : subtags :: Language :: UNKNOWN , script : None , region , variants : subtags :: Variants :: new () , } } }
};
}
