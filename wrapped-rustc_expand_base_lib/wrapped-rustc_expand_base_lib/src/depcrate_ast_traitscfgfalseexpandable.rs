// Generated macro for CfgFalseExpandable (trait)
macro_rules! Depcrate_ast_traitsCfgFalseExpandable {
() => {
// Module: crate::ast_traits
// Provides: {"CfgFalseExpandable"}
// Dependencies: {}
pub trait CfgFalseExpandable : Sized { fn expand_cfg_false < D : CfgFalseReporterContext > (& mut self , collector : & mut D , pos : usize , span : Span) ; }
};
}
