// Generated macro for DoublePlaceholderKey (enum)
macro_rules! Depcrate_doubleDoublePlaceholderKey {
() => {
// Module: crate::double
// Provides: {"DoublePlaceholderKey"}
// Dependencies: {}
# [doc = " A two-value enum for the [`DoublePlaceholder`] pattern backend."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::cmp::Ordering;"] # [doc = " use core::str::FromStr;"] # [doc = " use icu_pattern::DoublePlaceholderKey;"] # [doc = " use icu_pattern::DoublePlaceholderPattern;"] # [doc = " use icu_pattern::PatternItem;"] # [doc = ""] # [doc = " // Parse the string syntax"] # [doc = " let pattern = DoublePlaceholderPattern::try_from_str("] # [doc = "     \"Hello, {0} and {1}!\","] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     pattern.iter().cmp("] # [doc = "         ["] # [doc = "             PatternItem::Literal(\"Hello, \"),"] # [doc = "             PatternItem::Placeholder(DoublePlaceholderKey::Place0),"] # [doc = "             PatternItem::Literal(\" and \"),"] # [doc = "             PatternItem::Placeholder(DoublePlaceholderKey::Place1),"] # [doc = "             PatternItem::Literal(\"!\")"] # [doc = "         ]"] # [doc = "         .into_iter()"] # [doc = "     ),"] # [doc = "     Ordering::Equal"] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] # [allow (clippy :: exhaustive_enums)] pub enum DoublePlaceholderKey { # [doc = " The placeholder `{0}`."] Place0 , # [doc = " The placeholder `{1}`."] Place1 , }
};
}
