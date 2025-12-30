// Generated macro for impl_111 (impl)
macro_rules! Depcrate_errorbarimpl_111 {
() => {
// Module: crate::errorbar
// Provides: {"impl_111"}
// Dependencies: {}
impl < X , Y , L , H > ErrorBar < X , Y , L , H > { fn style (& self) -> Style { match * self { ErrorBar :: XErrorBars { .. } => Style :: XErrorBars , ErrorBar :: XErrorLines { .. } => Style :: XErrorLines , ErrorBar :: YErrorBars { .. } => Style :: YErrorBars , ErrorBar :: YErrorLines { .. } => Style :: YErrorLines , } } }
};
}
