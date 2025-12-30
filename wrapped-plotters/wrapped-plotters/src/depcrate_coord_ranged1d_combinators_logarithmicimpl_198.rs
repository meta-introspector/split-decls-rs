// Generated macro for impl_198 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_198 {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_198"}
// Dependencies: {}
impl < V : LogScalable > LogRangeExt < V > { # [doc = " Set the zero point of the log scale coordinate. Zero point is the point where we map -inf"] # [doc = " of the axis to the coordinate"] pub fn zero_point (mut self , value : V) -> Self where V : PartialEq , { self . zero = if V :: from_f64 (0.0) == value { 0.0 } else { value . as_f64 () } ; self } # [doc = " Set the base multiplier"] pub fn base (mut self , base : f64) -> Self { if self . base > 1.0 { self . base = base ; } self } }
};
}
