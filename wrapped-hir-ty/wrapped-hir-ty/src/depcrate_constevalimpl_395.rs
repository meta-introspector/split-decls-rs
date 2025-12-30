// Generated macro for impl_395 (impl)
macro_rules! Depcrate_constevalimpl_395 {
() => {
// Module: crate::consteval
// Provides: {"impl_395"}
// Dependencies: {}
impl From < MirLowerError > for ConstEvalError { fn from (value : MirLowerError) -> Self { match value { MirLowerError :: ConstEvalError (_ , e) => * e , _ => ConstEvalError :: MirLowerError (value) , } } }
};
}
