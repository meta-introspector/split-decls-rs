// Generated macro for FlattensOutputs (trait)
macro_rules! Depcrate_ast_traitsFlattensOutputs {
() => {
// Module: crate::ast_traits
// Provides: {"FlattensOutputs"}
// Dependencies: {}
pub trait FlattensOutputs : Sized { type FlatMapOutputTy ; fn flatten_outputs (_outputs : impl Iterator < Item = Self :: FlatMapOutputTy >) -> Self :: FlatMapOutputTy ; }
};
}
