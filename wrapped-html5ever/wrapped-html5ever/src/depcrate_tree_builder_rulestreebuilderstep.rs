// Generated macro for TreeBuilderStep (trait)
macro_rules! Depcrate_tree_builder_rulesTreeBuilderStep {
() => {
// Module: crate::tree_builder::rules
// Provides: {"TreeBuilderStep"}
// Dependencies: {}
pub trait TreeBuilderStep { fn step (& mut self , mode : InsertionMode , token : Token) -> ProcessResult ; fn step_foreign (& mut self , token : Token) -> ProcessResult ; }
};
}
