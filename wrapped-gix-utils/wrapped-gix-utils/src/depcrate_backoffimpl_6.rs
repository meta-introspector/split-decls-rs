// Generated macro for impl_6 (impl)
macro_rules! Depcrate_backoffimpl_6 {
() => {
// Module: crate::backoff
// Provides: {"impl_6"}
// Dependencies: {}
impl Quadratic < fn (usize) -> usize > { # [doc = " Create a new quadratic backoff iterator that backs off in randomized, ever increasing steps."] pub fn default_with_random () -> Self { Quadratic { multiplier : 1 , max_multiplier : 1000 , exponent : 1 , transform : randomize , } } }
};
}
