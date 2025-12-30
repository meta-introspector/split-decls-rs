// Generated macro for MultiNamedPlaceholderKey (struct)
macro_rules! Depcrate_multi_namedMultiNamedPlaceholderKey {
() => {
// Module: crate::multi_named
// Provides: {"MultiNamedPlaceholderKey"}
// Dependencies: {}
# [doc = " A string wrapper for the [`MultiNamedPlaceholder`] pattern backend."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::cmp::Ordering;"] # [doc = " use core::str::FromStr;"] # [doc = " use icu_pattern::MultiNamedPlaceholderKey;"] # [doc = " use icu_pattern::MultiNamedPlaceholderPattern;"] # [doc = " use icu_pattern::PatternItem;"] # [doc = ""] # [doc = " // Parse the string syntax and check the resulting data store:"] # [doc = " let pattern = MultiNamedPlaceholderPattern::try_from_str("] # [doc = "     \"Hello, {person0} and {person1}!\","] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     pattern.iter().cmp("] # [doc = "         ["] # [doc = "             PatternItem::Literal(\"Hello, \"),"] # [doc = "             PatternItem::Placeholder(MultiNamedPlaceholderKey(\"person0\")),"] # [doc = "             PatternItem::Literal(\" and \"),"] # [doc = "             PatternItem::Placeholder(MultiNamedPlaceholderKey(\"person1\")),"] # [doc = "             PatternItem::Literal(\"!\")"] # [doc = "         ]"] # [doc = "         .into_iter()"] # [doc = "     ),"] # [doc = "     Ordering::Equal"] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord)] # [repr (transparent)] # [allow (clippy :: exhaustive_structs)] pub struct MultiNamedPlaceholderKey < 'a > (pub & 'a str) ;
};
}
