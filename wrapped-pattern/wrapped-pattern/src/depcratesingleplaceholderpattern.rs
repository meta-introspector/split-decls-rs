// Generated macro for SinglePlaceholderPattern (type)
macro_rules! DepcrateSinglePlaceholderPattern {
() => {
// Module: crate
// Provides: {"SinglePlaceholderPattern"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::str::FromStr;"] # [doc = " use icu_pattern::SinglePlaceholderPattern;"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " // Create a pattern from the string syntax:"] # [doc = " let pattern = SinglePlaceholderPattern::try_from_str("] # [doc = "     \"Hello, {0}!\","] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " // Interpolate some values into the pattern:"] # [doc = " assert_writeable_eq!(pattern.interpolate([\"Alice\"]), \"Hello, Alice!\");"] # [doc = " ```"] pub type SinglePlaceholderPattern = Pattern < SinglePlaceholder > ;
};
}
