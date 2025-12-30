// Generated macro for Pass1 (struct)
macro_rules! Depcrate_transliterate_compile_pass1Pass1 {
() => {
// Module: crate::transliterate::compile::pass1
// Provides: {"Pass1"}
// Dependencies: {}
# [doc = " Responsible for the first pass as described in the module-level documentation."] # [derive (Debug , Clone)] pub (crate) struct Pass1 < 'p > { direction : Direction , forward_data : Pass1Data , reverse_data : Pass1Data , variable_data : BTreeMap < String , Pass1Data > , forward_filter : Option < parse :: FilterSet > , reverse_filter : Option < parse :: FilterSet > , forward_rule_group_agg : rule_group_agg :: ForwardRuleGroupAggregator < 'p > , reverse_rule_group_agg : rule_group_agg :: ReverseRuleGroupAggregator < 'p > , variable_definitions : BTreeMap < String , & 'p [parse :: Element] > , target_disallowed_variables : BTreeSet < String > , }
};
}
