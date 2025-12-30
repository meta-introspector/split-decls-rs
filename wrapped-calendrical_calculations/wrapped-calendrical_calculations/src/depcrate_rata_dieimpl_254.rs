// Generated macro for impl_254 (impl)
macro_rules! Depcrate_rata_dieimpl_254 {
() => {
// Module: crate::rata_die
// Provides: {"impl_254"}
// Dependencies: {}
impl Moment { # [doc = " Create a new moment"] pub const fn new (value : f64) -> Moment { Moment (value) } # [doc = " Get the inner field of a Moment"] pub const fn inner (self) -> f64 { self . 0 } # [doc = " Get the RataDie of a Moment"] pub fn as_rata_die (self) -> RataDie { RataDie :: new (self . 0 . floor () as i64) } }
};
}
