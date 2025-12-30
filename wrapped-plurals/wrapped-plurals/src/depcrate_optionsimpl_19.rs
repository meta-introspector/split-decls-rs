// Generated macro for impl_19 (impl)
macro_rules! Depcrate_optionsimpl_19 {
() => {
// Module: crate::options
// Provides: {"impl_19"}
// Dependencies: {}
impl PluralRulesOptions { # [doc = " Constructs a new [`PluralRulesOptions`] struct."] pub const fn default () -> Self { Self { rule_type : None } } # [doc = " Auguments the struct with the set [`PluralRuleType`]."] pub const fn with_type (mut self , rule_type : PluralRuleType) -> Self { self . rule_type = Some (rule_type) ; self } }
};
}
