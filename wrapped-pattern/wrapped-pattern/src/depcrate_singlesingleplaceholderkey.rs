// Generated macro for SinglePlaceholderKey (enum)
macro_rules! Depcrate_singleSinglePlaceholderKey {
() => {
// Module: crate::single
// Provides: {"SinglePlaceholderKey"}
// Dependencies: {}
# [doc = " A singleton enum for the [`SinglePlaceholder`] pattern backend."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::cmp::Ordering;"] # [doc = " use core::str::FromStr;"] # [doc = " use icu_pattern::PatternItem;"] # [doc = " use icu_pattern::SinglePlaceholder;"] # [doc = " use icu_pattern::SinglePlaceholderKey;"] # [doc = " use icu_pattern::SinglePlaceholderPattern;"] # [doc = ""] # [doc = " // Parse the string syntax and check the resulting data store:"] # [doc = " let pattern = SinglePlaceholderPattern::try_from_str("] # [doc = "     \"Hello, {0}!\","] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     pattern.iter().cmp("] # [doc = "         ["] # [doc = "             PatternItem::Literal(\"Hello, \"),"] # [doc = "             PatternItem::Placeholder(SinglePlaceholderKey::Singleton),"] # [doc = "             PatternItem::Literal(\"!\")"] # [doc = "         ]"] # [doc = "         .into_iter()"] # [doc = "     ),"] # [doc = "     Ordering::Equal"] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] # [allow (clippy :: exhaustive_enums)] pub enum SinglePlaceholderKey { Singleton , }
};
}
