// Generated macro for impl_206 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_206 {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_206"}
// Dependencies: {}
# [allow (deprecated)] impl < V : LogScalable > From < LogRange < V > > for LogCoord < V > { fn from (range : LogRange < V >) -> LogCoord < V > { range . 0 . log_scale () . into () } }
};
}
