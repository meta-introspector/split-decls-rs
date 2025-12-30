// Generated macro for impl_67 (impl)
macro_rules! Depcrate_localeimpl_67 {
() => {
// Module: crate::locale
// Provides: {"impl_67"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::Locale;"] # [doc = " use icu::locale::{locale, subtags::region};"] # [doc = ""] # [doc = " assert_eq!(Locale::from(Some(region!(\"US\"))), locale!(\"und-US\"));"] # [doc = " ```"] impl From < Option < subtags :: Region > > for Locale { fn from (region : Option < subtags :: Region >) -> Self { Self { id : region . into () , extensions : extensions :: Extensions :: new () , } } }
};
}
