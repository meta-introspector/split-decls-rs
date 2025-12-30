// Generated macro for impl_65 (impl)
macro_rules! Depcrate_localeimpl_65 {
() => {
// Module: crate::locale
// Provides: {"impl_65"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::Locale;"] # [doc = " use icu::locale::{locale, subtags::language};"] # [doc = ""] # [doc = " assert_eq!(Locale::from(language!(\"en\")), locale!(\"en\"));"] # [doc = " ```"] impl From < subtags :: Language > for Locale { fn from (language : subtags :: Language) -> Self { Self { id : language . into () , extensions : extensions :: Extensions :: new () , } } }
};
}
