// Generated macro for InterpolatorStatus (struct)
macro_rules! Depcrate_hyperlinkInterpolatorStatus {
() => {
// Module: crate::hyperlink
// Provides: {"InterpolatorStatus"}
// Dependencies: {}
# [doc = " A status indicating whether a hyperlink was written or not."] # [doc = ""] # [doc = " This is created by `Interpolator::begin` and used by `Interpolator::finish`"] # [doc = " to determine whether a hyperlink was actually opened or not. If it wasn't"] # [doc = " opened, then finishing interpolation is a no-op."] # [derive (Debug)] pub (crate) struct InterpolatorStatus { active : bool , }
};
}
