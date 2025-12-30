// Generated macro for impl_68 (impl)
macro_rules! Depcrate_localeimpl_68 {
() => {
// Module: crate::locale
// Provides: {"impl_68"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::Locale;"] # [doc = " use icu::locale::{"] # [doc = "     locale,"] # [doc = "     subtags::{language, region, script},"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     Locale::from(("] # [doc = "         language!(\"en\"),"] # [doc = "         Some(script!(\"Latn\")),"] # [doc = "         Some(region!(\"US\"))"] # [doc = "     )),"] # [doc = "     locale!(\"en-Latn-US\")"] # [doc = " );"] # [doc = " ```"] impl From < (subtags :: Language , Option < subtags :: Script > , Option < subtags :: Region > ,) > for Locale { fn from (lsr : (subtags :: Language , Option < subtags :: Script > , Option < subtags :: Region > ,) ,) -> Self { Self { id : lsr . into () , extensions : extensions :: Extensions :: new () , } } }
};
}
