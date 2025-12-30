// Generated macro for select_arm (function)
macro_rules! Depcrate_cfg_selectselect_arm {
() => {
// Module: crate::cfg_select
// Provides: {"select_arm"}
// Dependencies: {}
# [doc = " Selects the first arm whose predicate evaluates to true."] fn select_arm (ecx : & ExtCtxt < '_ > , branches : CfgSelectBranches) -> Option < (TokenStream , Span) > { for (cfg , tt , arm_span) in branches . reachable { if attr :: cfg_matches (& cfg , & ecx . sess , ecx . current_expansion . lint_node_id , Some (ecx . ecfg . features) ,) { return Some ((tt , arm_span)) ; } } branches . wildcard . map (| (_ , tt , span) | (tt , span)) }
};
}
