// Generated macro for PluralRuleType (enum)
macro_rules! Depcrate_optionsPluralRuleType {
() => {
// Module: crate::options
// Provides: {"PluralRuleType"}
// Dependencies: {}
# [doc = " A type of a plural rule which can be associated with the [`PluralRules`] struct."] # [doc = ""] # [doc = " [`PluralRules`]: crate::PluralRules"] # [derive (Debug , PartialEq , Eq , Clone , Copy , Hash , Default)] # [non_exhaustive] pub enum PluralRuleType { # [doc = " Cardinal plural forms express quantities of units such as time, currency or distance,"] # [doc = " used in conjunction with a number expressed in decimal digits (i.e. \"2\", not \"two\")."] # [doc = ""] # [doc = " For example, English has two forms for cardinals:"] # [doc = ""] # [doc = " * [`One`]: `1 day`"] # [doc = " * [`Other`]: `0 days`, `2 days`, `10 days`, `0.3 days`"] # [doc = ""] # [doc = " [`One`]: crate::PluralCategory::One"] # [doc = " [`Other`]: crate::PluralCategory::Other"] # [default] Cardinal , # [doc = " Ordinal plural forms denote the order of items in a set and are always integers."] # [doc = ""] # [doc = " For example, English has four forms for ordinals:"] # [doc = ""] # [doc = " * [`One`]: `1st floor`, `21st floor`, `101st floor`"] # [doc = " * [`Two`]: `2nd floor`, `22nd floor`, `102nd floor`"] # [doc = " * [`Few`]: `3rd floor`, `23rd floor`, `103rd floor`"] # [doc = " * [`Other`]: `4th floor`, `11th floor`, `96th floor`"] # [doc = ""] # [doc = " [`One`]: crate::PluralCategory::One"] # [doc = " [`Two`]: crate::PluralCategory::Two"] # [doc = " [`Few`]: crate::PluralCategory::Few"] # [doc = " [`Other`]: crate::PluralCategory::Other"] Ordinal , }
};
}
