// Generated macro for impl_66 (impl)
macro_rules! Depcrate_localeimpl_66 {
() => {
// Module: crate::locale
// Provides: {"impl_66"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::Locale;"] # [doc = " use icu::locale::{locale, subtags::script};"] # [doc = ""] # [doc = " assert_eq!(Locale::from(Some(script!(\"latn\"))), locale!(\"und-Latn\"));"] # [doc = " ```"] impl From < Option < subtags :: Script > > for Locale { fn from (script : Option < subtags :: Script >) -> Self { Self { id : script . into () , extensions : extensions :: Extensions :: new () , } } }
};
}
