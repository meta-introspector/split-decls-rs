// Generated macro for impl_48 (impl)
macro_rules! Depcrate_langidimpl_48 {
() => {
// Module: crate::langid
// Provides: {"impl_48"}
// Dependencies: {}
# [doc = " Convert from a [`LanguageIdentifier`] to an LSR tuple."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::{"] # [doc = "     langid,"] # [doc = "     subtags::{language, region, script},"] # [doc = " };"] # [doc = ""] # [doc = " let lid = langid!(\"en-Latn-US\");"] # [doc = " let (lang, script, region) = (&lid).into();"] # [doc = ""] # [doc = " assert_eq!(lang, language!(\"en\"));"] # [doc = " assert_eq!(script, Some(script!(\"Latn\")));"] # [doc = " assert_eq!(region, Some(region!(\"US\")));"] # [doc = " ```"] impl From < & LanguageIdentifier > for (subtags :: Language , Option < subtags :: Script > , Option < subtags :: Region > ,) { fn from (langid : & LanguageIdentifier) -> Self { (langid . language , langid . script , langid . region) } }
};
}
