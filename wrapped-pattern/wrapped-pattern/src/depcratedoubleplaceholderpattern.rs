// Generated macro for DoublePlaceholderPattern (type)
macro_rules! DepcrateDoublePlaceholderPattern {
() => {
// Module: crate
// Provides: {"DoublePlaceholderPattern"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::str::FromStr;"] # [doc = " use icu_pattern::DoublePlaceholderPattern;"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " // Create a pattern from the string syntax:"] # [doc = " let pattern = DoublePlaceholderPattern::try_from_str("] # [doc = "     \"Hello, {0} and {1}!\","] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " // Interpolate some values into the pattern:"] # [doc = " assert_writeable_eq!("] # [doc = "     pattern.interpolate([\"Alice\", \"Bob\"]),"] # [doc = "     \"Hello, Alice and Bob!\""] # [doc = " );"] # [doc = " ```"] pub type DoublePlaceholderPattern = Pattern < DoublePlaceholder > ;
};
}
