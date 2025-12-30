// Generated macro for impl_44 (impl)
macro_rules! Depcrate_langidimpl_44 {
() => {
// Module: crate::langid
// Provides: {"impl_44"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::{langid, subtags::language, LanguageIdentifier};"] # [doc = ""] # [doc = " assert_eq!(LanguageIdentifier::from(language!(\"en\")), langid!(\"en\"));"] # [doc = " ```"] impl From < subtags :: Language > for LanguageIdentifier { fn from (language : subtags :: Language) -> Self { Self { language , script : None , region : None , variants : subtags :: Variants :: new () , } } }
};
}
