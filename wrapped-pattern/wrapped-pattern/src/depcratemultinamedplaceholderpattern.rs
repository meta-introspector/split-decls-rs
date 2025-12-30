// Generated macro for MultiNamedPlaceholderPattern (type)
macro_rules! DepcrateMultiNamedPlaceholderPattern {
() => {
// Module: crate
// Provides: {"MultiNamedPlaceholderPattern"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::str::FromStr;"] # [doc = " use icu_pattern::MultiNamedPlaceholderPattern;"] # [doc = " use std::collections::BTreeMap;"] # [doc = " use writeable::assert_try_writeable_eq;"] # [doc = ""] # [doc = " // Create a pattern from the string syntax:"] # [doc = " let pattern = MultiNamedPlaceholderPattern::try_from_str("] # [doc = "     \"Hello, {person0} and {person1}!\","] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " // Interpolate some values into the pattern:"] # [doc = " assert_try_writeable_eq!("] # [doc = "     pattern.try_interpolate("] # [doc = "         [(\"person0\", \"Alice\"), (\"person1\", \"Bob\")]"] # [doc = "             .into_iter()"] # [doc = "             .collect::<BTreeMap<&str, &str>>()"] # [doc = "     ),"] # [doc = "     \"Hello, Alice and Bob!\""] # [doc = " );"] # [doc = " ```"] pub type MultiNamedPlaceholderPattern = Pattern < MultiNamedPlaceholder > ;
};
}
