// Generated macro for impl_460 (impl)
macro_rules! Depcrate_constevalimpl_460 {
() => {
// Module: crate::consteval
// Provides: {"impl_460"}
// Dependencies: {}
impl < 'db > From < MirLowerError < 'db > > for ConstEvalError < 'db > { fn from (value : MirLowerError < 'db >) -> Self { match value { MirLowerError :: ConstEvalError (_ , e) => * e , _ => ConstEvalError :: MirLowerError (value) , } } }
};
}
