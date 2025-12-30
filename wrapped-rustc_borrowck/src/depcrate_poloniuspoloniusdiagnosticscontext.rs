// Generated macro for PoloniusDiagnosticsContext (struct)
macro_rules! Depcrate_poloniusPoloniusDiagnosticsContext {
() => {
// Module: crate::polonius
// Provides: {"PoloniusDiagnosticsContext"}
// Dependencies: {}
# [doc = " This struct holds the data needed by the borrowck error computation and diagnostics. Its data is"] # [doc = " computed from the [PoloniusContext] when computing NLL regions."] pub (crate) struct PoloniusDiagnosticsContext { # [doc = " The localized outlives constraints that were computed in the main analysis."] localized_outlives_constraints : LocalizedOutlivesConstraintSet , # [doc = " The liveness data computed during MIR typeck: [PoloniusLivenessContext::boring_nll_locals]."] pub (crate) boring_nll_locals : FxHashSet < Local > , }
};
}
