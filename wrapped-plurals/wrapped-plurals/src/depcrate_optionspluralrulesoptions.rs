// Generated macro for PluralRulesOptions (struct)
macro_rules! Depcrate_optionsPluralRulesOptions {
() => {
// Module: crate::options
// Provides: {"PluralRulesOptions"}
// Dependencies: {}
# [doc = " A list of options set by the developer to adjust the behavior of the PluralRules."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use icu::plurals::{PluralRuleType, PluralRulesOptions};"] # [doc = ""] # [doc = " let options ="] # [doc = "     PluralRulesOptions::default().with_type(PluralRuleType::Cardinal);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub struct PluralRulesOptions { # [doc = " Plural rule type to use."] # [doc = ""] # [doc = " Default is [`PluralRuleType::Cardinal`]"] pub rule_type : Option < PluralRuleType > , }
};
}
