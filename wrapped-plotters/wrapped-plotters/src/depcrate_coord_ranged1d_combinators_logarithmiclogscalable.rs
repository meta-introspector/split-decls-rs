// Generated macro for LogScalable (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicLogScalable {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"LogScalable"}
// Dependencies: {}
# [doc = " The trait for the type that is able to be presented in the log scale."] # [doc = " This trait is primarily used by [LogRangeExt](struct.LogRangeExt.html)."] pub trait LogScalable : Clone { # [doc = " Make the conversion from the type to the floating point number"] fn as_f64 (& self) -> f64 ; # [doc = " Convert a floating point number to the scale"] fn from_f64 (f : f64) -> Self ; }
};
}
