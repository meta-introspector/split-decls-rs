// Generated macro for IntoLogRange (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicIntoLogRange {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"IntoLogRange"}
// Dependencies: {}
# [doc = " Convert a range to a log scale coordinate spec"] pub trait IntoLogRange { # [doc = " The type of the value"] type ValueType : LogScalable ; # [doc = " Make the log scale coordinate"] fn log_scale (self) -> LogRangeExt < Self :: ValueType > ; }
};
}
