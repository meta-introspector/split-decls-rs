// Generated macro for CfgFalseReporter (trait)
macro_rules! Depcrate_cfg_false_reporterCfgFalseReporter {
() => {
// Module: crate::cfg_false_reporter
// Provides: {"CfgFalseReporter"}
// Dependencies: {}
pub trait CfgFalseReporter { fn report_cfg_false < N : HasAttrs + HasNodeId > (& mut self , node : & mut N , attr_span : Span , attr_pos : usize) ; }
};
}
