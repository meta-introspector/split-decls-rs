// Generated macro for DivergingFallbackBehavior (enum)
macro_rules! Depcrate_fallbackDivergingFallbackBehavior {
() => {
// Module: crate::fallback
// Provides: {"DivergingFallbackBehavior"}
// Dependencies: {}
# [derive (Copy , Clone)] pub (crate) enum DivergingFallbackBehavior { # [doc = " Always fallback to `()` (aka \"always spontaneous decay\")"] ToUnit , # [doc = " Sometimes fallback to `!`, but mainly fallback to `()` so that most of the crates are not broken."] ContextDependent , # [doc = " Always fallback to `!` (which should be equivalent to never falling back + not making"] # [doc = " never-to-any coercions unless necessary)"] ToNever , # [doc = " Don't fallback at all"] NoFallback , }
};
}
