// Generated macro for impl_195 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_195 {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_195"}
// Dependencies: {}
impl < T : LogScalable > IntoLogRange for Range < T > { type ValueType = T ; fn log_scale (self) -> LogRangeExt < T > { LogRangeExt { range : self , zero : 0.0 , base : 10.0 , } } }
};
}
