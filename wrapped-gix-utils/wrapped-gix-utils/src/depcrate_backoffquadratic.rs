// Generated macro for Quadratic (struct)
macro_rules! Depcrate_backoffQuadratic {
() => {
// Module: crate::backoff
// Provides: {"Quadratic"}
// Dependencies: {}
# [doc = " A utility to calculate steps for quadratic backoff similar to how it's done in `git`."] pub struct Quadratic < Fn > { multiplier : usize , max_multiplier : usize , exponent : usize , transform : Fn , }
};
}
