// Generated macro for ReverseRuleGroupAggregator (struct)
macro_rules! Depcrate_transliterate_compile_rule_group_aggReverseRuleGroupAggregator {
() => {
// Module: crate::transliterate::compile::rule_group_agg
// Provides: {"ReverseRuleGroupAggregator"}
// Dependencies: {}
# [derive (Debug , Clone)] pub (crate) struct ReverseRuleGroupAggregator < 'p > { current : ReverseRuleGroup < 'p > , groups : VecDeque < (Vec < Cow < 'p , parse :: SingleId > > , Vec < UniConversionRule < 'p > >) > , preceding_conversion_group : Option < Vec < UniConversionRule < 'p > > > , }
};
}
