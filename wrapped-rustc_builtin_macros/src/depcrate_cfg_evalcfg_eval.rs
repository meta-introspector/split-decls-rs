// Generated macro for cfg_eval (function)
macro_rules! Depcrate_cfg_evalcfg_eval {
() => {
// Module: crate::cfg_eval
// Provides: {"cfg_eval"}
// Dependencies: {}
pub (crate) fn cfg_eval (sess : & Session , features : & Features , annotatable : Annotatable , lint_node_id : NodeId ,) -> Annotatable { let features = Some (features) ; CfgEval (StripUnconfigured { sess , features , config_tokens : true , lint_node_id }) . configure_annotatable (annotatable) }
};
}
