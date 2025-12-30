// Generated macro for impl_477 (impl)
macro_rules! Depcrate_contextimpl_477 {
() => {
// Module: crate::context
// Provides: {"impl_477"}
// Dependencies: {}
impl ShouldEmit { pub (crate) fn emit_err (& self , diag : Diag < '_ >) -> ErrorGuaranteed { match self { ShouldEmit :: EarlyFatal { .. } if diag . level () == Level :: DelayedBug => diag . emit () , ShouldEmit :: EarlyFatal { .. } => diag . upgrade_to_fatal () . emit () , ShouldEmit :: ErrorsAndLints => diag . emit () , ShouldEmit :: Nothing => diag . delay_as_bug () , } } }
};
}
